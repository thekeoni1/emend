//! A workbook's pivot tables and their caches, read as written
//! (`specs/0033-read-breadth-survey.md`, "Pivot tables"). The Cargo feature
//! `read`.
//!
//! [`Package::pivot_tables`] finds every pivot table part by its content
//! type, in entry order, so a table the workbook part relates is found
//! beside those its sheets relate; the sheet is joined afterwards from the
//! sheets' own `pivotTable` relationships. Each part, and the cache
//! definition its first `pivotCacheDefinition` relationship names, is
//! streamed once: the attributes and the four lists the types carry are
//! kept and every other child is walked past with its subtree, so what is
//! held is bounded by the fields, not by the cached records. The records
//! part is never opened, and a `cacheId` is reported as written, not joined
//! to the workbook's `<pivotCaches>`.

use std::io::{Read, Seek};

use quick_xml::XmlVersion;
use quick_xml::events::{BytesStart, Event};

use super::{
    MAIN, NO_SUCH_PART, PIVOT_CACHE_DEFINITION, PIVOT_TABLE, Package, SheetKind,
    is_relationships_attribute,
};
use crate::archive::Error;
use crate::cells::{Area, skip_subtree};
use crate::xml::{Scope, Stream};

/// What a pivot table or cache definition this layer cannot read into the
/// types is refused with; [`crate::package`]'s catalogue lists it.
pub(crate) const PIVOT_UNREADABLE: Error = Error::Package("a pivot table cannot be read");

/// The pivot table part's content type.
const PIVOT_TABLE_TYPE: &[u8] =
    b"application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml";

/// One pivot table: its part, the sheet that relates it, what its root and
/// its lists say, and its cache. The Cargo feature `read`.
#[derive(Clone, Debug, PartialEq)]
pub struct PivotTable {
    /// The pivot table part's entry name.
    pub part: Vec<u8>,
    /// The part of the first worksheet or chartsheet of
    /// [`Workbook::sheets`](super::Workbook::sheets) whose relationships
    /// name this part through a `pivotTable` relationship; `None` when no
    /// sheet does (a table the workbook part relates).
    pub sheet: Option<Vec<u8>>,
    /// The root's `name`, unescaped.
    pub name: String,
    /// The root's `cacheId`, as written.
    pub cache_id: u32,
    /// `<location>`'s `ref`.
    pub location: Area,
    /// `<location>`'s `firstHeaderRow`.
    pub first_header_row: u32,
    /// `<location>`'s `firstDataRow`.
    pub first_data_row: u32,
    /// `<location>`'s `firstDataCol`.
    pub first_data_col: u32,
    /// The cache definition the part's first `pivotCacheDefinition`
    /// relationship names; `None` without one.
    pub cache: Option<PivotCache>,
    /// The `pivotField` children of `<pivotFields>`, in order.
    pub fields: Vec<PivotField>,
    /// The `field` children of `<rowFields>`, in order.
    pub row_fields: Vec<FieldRef>,
    /// The `field` children of `<colFields>`, in order.
    pub column_fields: Vec<FieldRef>,
    /// The `pageField` children of `<pageFields>`, in order.
    pub page_fields: Vec<FieldRef>,
    /// The `dataField` children of `<dataFields>`, in order.
    pub data_fields: Vec<DataField>,
}

/// One `pivotField`. The Cargo feature `read`.
#[derive(Clone, Debug, PartialEq)]
pub struct PivotField {
    /// The cache field at the same position; `None` without a cache or past
    /// its fields. A `pivotField`'s own `name` is not read.
    pub name: Option<String>,
    /// `axis`.
    pub axis: Option<Axis>,
    /// `dataField`, false when absent.
    pub data_field: bool,
}

/// A `pivotField`'s `axis`. The Cargo feature `read`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    /// `axisRow`.
    Row,
    /// `axisCol`.
    Column,
    /// `axisPage`.
    Page,
    /// `axisValues`.
    Values,
}

/// A `field`'s `x` or a `pageField`'s `fld`. The Cargo feature `read`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldRef {
    /// A field by its position in [`PivotTable::fields`].
    Field(u32),
    /// `-2`, the values field.
    Values,
}

/// One `dataField`. The Cargo feature `read`.
#[derive(Clone, Debug, PartialEq)]
pub struct DataField {
    /// `name`, unescaped.
    pub name: Option<String>,
    /// `fld`.
    pub field: u32,
    /// `subtotal`, [`Subtotal::Sum`] when absent.
    pub subtotal: Subtotal,
}

/// A `dataField`'s `subtotal`. The Cargo feature `read`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Subtotal {
    /// `sum`, and the default.
    Sum,
    /// `count`.
    Count,
    /// `average`.
    Average,
    /// `max`.
    Max,
    /// `min`.
    Min,
    /// `product`.
    Product,
    /// `countNums`.
    CountNums,
    /// `stdDev`.
    StdDev,
    /// `stdDevp`.
    StdDevP,
    /// `var`.
    Var,
    /// `varp`.
    VarP,
}

/// A pivot cache definition. The Cargo feature `read`.
#[derive(Clone, Debug, PartialEq)]
pub struct PivotCache {
    /// The cache definition's entry name.
    pub part: Vec<u8>,
    /// Its `cacheSource`.
    pub source: PivotSource,
    /// Its `cacheField` names, in order, unescaped.
    pub fields: Vec<String>,
    /// Its `recordCount`.
    pub record_count: Option<u64>,
}

/// A cache's `cacheSource`. The Cargo feature `read`.
#[derive(Clone, Debug, PartialEq)]
pub enum PivotSource {
    /// `type="worksheet"`, with its `worksheetSource`'s `sheet`, `ref` and
    /// `name` as written; its `r:id`, an external workbook, is not followed.
    Worksheet {
        /// `sheet`, unescaped.
        sheet: Option<String>,
        /// `ref`.
        reference: Option<Area>,
        /// `name`, unescaped.
        name: Option<String>,
    },
    /// `type="external"`.
    External {
        /// `connectionId`.
        connection_id: Option<u32>,
    },
    /// `type="consolidation"`.
    Consolidation,
    /// `type="scenario"`.
    Scenario,
}

impl<R: Read + Seek> Package<R> {
    /// Every pivot table of the package, read once and lent
    /// (`specs/0033-read-breadth-survey.md`, "Pivot tables"): each entry
    /// whose content type is the pivot table's, in entry order, with the
    /// cache definition its own first `pivotCacheDefinition` relationship
    /// names. The Cargo feature `read`.
    ///
    /// # Errors
    ///
    /// [`Error::Package`] "a pivot table cannot be read" for a root other
    /// than `pivotTableDefinition` or `pivotCacheDefinition` in the main
    /// namespace; no `name`, no `cacheId`, no `location` or one lacking any
    /// of its four attributes; a `ref` that is not an area; an `axis`,
    /// `subtotal` or `type` outside its enumeration; an `x`, `fld`,
    /// `cacheId`, `connectionId`, a `location` row or column, a
    /// `recordCount` or a `dataField` flag that does not parse as its type
    /// (`x` and a `pageField`'s `fld` are `-2` or a `u32`); a `dataField` or
    /// `pageField` `fld`, or a `field` `x`, past the table's fields; a
    /// `cacheSource` with no `type`; a `cacheField` with no `name`; two
    /// `location`, `pivotFields`, `cacheSource` or `cacheFields` children.
    /// "no such part" for a `pivotCacheDefinition` relationship naming no
    /// entry; whatever [`Package::content_types`], [`Package::workbook`] and
    /// [`Package::relationships`] refuse; "an element or attribute uses a
    /// prefix its document does not bind"; [`Error::Xml`] for bytes that are
    /// not well-formed XML or not UTF-8.
    pub fn pivot_tables(&mut self) -> Result<&[PivotTable], Error> {
        if self.pivot_tables.is_none() {
            let tables = self.read_pivot_tables()?;
            self.pivot_tables = Some(tables);
        }
        // Filled above.
        Ok(self.pivot_tables.as_deref().unwrap())
    }

    /// The body of [`Package::pivot_tables`], before the list is kept.
    fn read_pivot_tables(&mut self) -> Result<Vec<PivotTable>, Error> {
        self.content_types()?;
        let parts: Vec<Vec<u8>> = {
            // Filled by the call above.
            let types = self.content_types.as_ref().unwrap();
            self.archive
                .entries()
                .iter()
                .map(|entry| entry.name())
                .filter(|name| types.of(name) == Some(PIVOT_TABLE_TYPE))
                .map(<[u8]>::to_vec)
                .collect()
        };
        if parts.is_empty() {
            return Ok(Vec::new());
        }
        // Each sheet's pivot table targets, in `<sheets>` order.
        let sheets: Vec<Vec<u8>> = self
            .workbook()?
            .sheets
            .iter()
            .filter(|sheet| matches!(sheet.kind, SheetKind::Worksheet | SheetKind::Chartsheet))
            .map(|sheet| sheet.part.clone())
            .collect();
        let mut related: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        for sheet in sheets {
            let rels = self.relationships(&sheet)?;
            for rel in rels.all() {
                if rel.external || !PIVOT_TABLE.contains(&rel.kind.as_slice()) {
                    continue;
                }
                related.push((rels.target_part(rel)?, sheet.clone()));
            }
        }
        let mut tables = Vec::with_capacity(parts.len());
        for part in parts {
            let cache_part = {
                let rels = self.relationships(&part)?;
                match rels.first_of(PIVOT_CACHE_DEFINITION) {
                    Some(rel) => Some(rels.target_part(rel)?),
                    None => None,
                }
            };
            let cache = match cache_part {
                Some(cache_part) => {
                    let index = self.archive.index_of(&cache_part).ok_or(NO_SUCH_PART)?;
                    let mut stream = Stream::new(self.archive.read(index)?, index)?;
                    Some(read_cache(&mut stream, index, cache_part)?)
                }
                None => None,
            };
            // Present: the name came from the entry list.
            let index = self.archive.index_of(&part).unwrap();
            let mut stream = Stream::new(self.archive.read(index)?, index)?;
            let sheet = related
                .iter()
                .find(|(target, _)| *target == part)
                .map(|(_, sheet)| sheet.clone());
            tables.push(read_table(&mut stream, index, part, sheet, cache)?);
        }
        Ok(tables)
    }
}

/// An element met: whether it is in the main namespace, its local name, its
/// unprefixed attributes unescaped and normalised, its `r:id` (either form of
/// the relationships namespace), and whether it was an empty-element tag.
/// The external link reader walks its parts with the same helpers.
pub(super) struct Element {
    main: bool,
    local: Vec<u8>,
    attributes: Vec<(Vec<u8>, String)>,
    relationship_id: Option<String>,
    pub(super) empty: bool,
}

impl Element {
    fn of(start: &BytesStart<'_>, scope: &Scope, index: usize, empty: bool) -> Result<Self, Error> {
        let (namespace, local) = scope.resolve_element(start.name())?;
        let main = MAIN.contains(&namespace.unwrap_or_default());
        let local = local.to_vec();
        let mut attributes = Vec::new();
        let mut relationship_id = None;
        for attribute in start.attributes() {
            let attribute = attribute.map_err(|_| Error::Xml(index))?;
            if attribute.key.as_namespace_binding().is_some() {
                continue;
            }
            let (namespace, name) = scope.resolve_attribute(attribute.key)?;
            let relationship = is_relationships_attribute(namespace) && name == b"id";
            if namespace.is_none() || relationship {
                let value = attribute
                    .normalized_value(XmlVersion::Implicit1_0)
                    .map_err(|_| Error::Xml(index))?
                    .into_owned();
                if relationship {
                    relationship_id = Some(value);
                } else {
                    attributes.push((name.to_vec(), value));
                }
            }
        }
        Ok(Self {
            main,
            local,
            attributes,
            relationship_id,
            empty,
        })
    }

    /// Whether this is the main-namespace element `local`.
    pub(super) fn is(&self, local: &[u8]) -> bool {
        self.main && self.local == local
    }

    /// Its `r:id`, as written.
    pub(super) fn relationship_id(&self) -> Option<&str> {
        self.relationship_id.as_deref()
    }

    pub(super) fn value(&self, name: &[u8]) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }

    /// The attribute `name` parsed as `T`; refused when it does not parse.
    fn number<T: std::str::FromStr>(&self, name: &[u8]) -> Result<Option<T>, Error> {
        self.value(name)
            .map(|value| value.parse().map_err(|_| PIVOT_UNREADABLE))
            .transpose()
    }

    /// The attribute `name` parsed as `T`; refused when absent.
    fn required<T: std::str::FromStr>(&self, name: &[u8]) -> Result<T, Error> {
        self.number(name)?.ok_or(PIVOT_UNREADABLE)
    }
}

/// The root element of a part, its start given to `scope`.
pub(super) fn root<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
) -> Result<Element, Error> {
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                scope.start(start)?;
                return Element::of(start, scope, index, empty);
            }
            _ => {}
        }
    }
    Err(Error::Xml(index))
}

/// The next child of the element whose start was given to `scope` last and
/// not yet ended, its start given to `scope` in turn; `None` at the parent's
/// end tag, which is consumed and given back to `scope`. A parent met as an
/// empty-element tag has no children: the caller asks only when it is not.
pub(super) fn child<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
) -> Result<Option<Element>, Error> {
    while let Some(token) = stream.next()? {
        match token.event {
            Event::Start(ref start) | Event::Empty(ref start) => {
                let empty = matches!(token.event, Event::Empty(_));
                scope.start(start)?;
                return Element::of(start, scope, index, empty).map(Some);
            }
            Event::End(_) => {
                scope.end();
                return Ok(None);
            }
            _ => {}
        }
    }
    Err(Error::Xml(index))
}

/// Walks past `element`'s subtree, through its end tag.
pub(super) fn past<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    element: &Element,
) -> Result<(), Error> {
    if element.empty {
        scope.end();
        Ok(())
    } else {
        skip_subtree(stream, scope, index)
    }
}

/// The children of `list` that are the main-namespace element `item`, in
/// order, each given to `each` and then walked past; every other child is
/// walked past unread.
pub(super) fn items<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    list: &Element,
    item: &[u8],
    mut each: impl FnMut(&Element) -> Result<(), Error>,
) -> Result<(), Error> {
    if list.empty {
        scope.end();
        return Ok(());
    }
    while let Some(element) = child(stream, scope, index)? {
        if element.is(item) {
            each(&element)?;
        }
        past(stream, scope, index, &element)?;
    }
    Ok(())
}

/// A `field`'s `x` or a `pageField`'s `fld`: `-2` the values field, else a
/// `u32`.
fn field_ref(value: Option<&str>) -> Result<FieldRef, Error> {
    let value = value.ok_or(PIVOT_UNREADABLE)?;
    if value == "-2" {
        return Ok(FieldRef::Values);
    }
    value
        .parse()
        .map(FieldRef::Field)
        .map_err(|_| PIVOT_UNREADABLE)
}

/// An `xsd:boolean`.
fn boolean(value: Option<&str>) -> Result<Option<bool>, Error> {
    match value {
        None => Ok(None),
        Some("1" | "true") => Ok(Some(true)),
        Some("0" | "false") => Ok(Some(false)),
        Some(_) => Err(PIVOT_UNREADABLE),
    }
}

/// Reads a `pivotTableDefinition` part.
fn read_table<R: Read>(
    stream: &mut Stream<R>,
    index: usize,
    part: Vec<u8>,
    sheet: Option<Vec<u8>>,
    cache: Option<PivotCache>,
) -> Result<PivotTable, Error> {
    let mut scope = Scope::new(index);
    let root = root(stream, &mut scope, index)?;
    if !root.is(b"pivotTableDefinition") {
        return Err(PIVOT_UNREADABLE);
    }
    let name = root.value(b"name").ok_or(PIVOT_UNREADABLE)?.to_owned();
    let cache_id = root.required(b"cacheId")?;
    let mut location = None;
    let mut fields: Option<Vec<PivotField>> = None;
    let mut row_fields = Vec::new();
    let mut column_fields = Vec::new();
    let mut page_fields = Vec::new();
    let mut data_fields = Vec::new();
    if !root.empty {
        while let Some(element) = child(stream, &mut scope, index)? {
            if element.is(b"location") {
                if location.is_some() {
                    return Err(PIVOT_UNREADABLE);
                }
                let reference = element.value(b"ref").ok_or(PIVOT_UNREADABLE)?;
                location = Some((
                    Area::parse(reference.as_bytes()).ok_or(PIVOT_UNREADABLE)?,
                    element.required(b"firstHeaderRow")?,
                    element.required(b"firstDataRow")?,
                    element.required(b"firstDataCol")?,
                ));
                past(stream, &mut scope, index, &element)?;
            } else if element.is(b"pivotFields") {
                if fields.is_some() {
                    return Err(PIVOT_UNREADABLE);
                }
                let mut list = Vec::new();
                items(
                    stream,
                    &mut scope,
                    index,
                    &element,
                    b"pivotField",
                    |field| {
                        let axis = match field.value(b"axis") {
                            None => None,
                            Some("axisRow") => Some(Axis::Row),
                            Some("axisCol") => Some(Axis::Column),
                            Some("axisPage") => Some(Axis::Page),
                            Some("axisValues") => Some(Axis::Values),
                            Some(_) => return Err(PIVOT_UNREADABLE),
                        };
                        let name = cache
                            .as_ref()
                            .and_then(|cache| cache.fields.get(list.len()))
                            .cloned();
                        list.push(PivotField {
                            name,
                            axis,
                            data_field: boolean(field.value(b"dataField"))?.unwrap_or(false),
                        });
                        Ok(())
                    },
                )?;
                fields = Some(list);
            } else if element.is(b"rowFields") {
                items(stream, &mut scope, index, &element, b"field", |field| {
                    row_fields.push(field_ref(field.value(b"x"))?);
                    Ok(())
                })?;
            } else if element.is(b"colFields") {
                items(stream, &mut scope, index, &element, b"field", |field| {
                    column_fields.push(field_ref(field.value(b"x"))?);
                    Ok(())
                })?;
            } else if element.is(b"pageFields") {
                items(stream, &mut scope, index, &element, b"pageField", |field| {
                    page_fields.push(field_ref(field.value(b"fld"))?);
                    Ok(())
                })?;
            } else if element.is(b"dataFields") {
                items(stream, &mut scope, index, &element, b"dataField", |field| {
                    let subtotal = match field.value(b"subtotal") {
                        None | Some("sum") => Subtotal::Sum,
                        Some("count") => Subtotal::Count,
                        Some("average") => Subtotal::Average,
                        Some("max") => Subtotal::Max,
                        Some("min") => Subtotal::Min,
                        Some("product") => Subtotal::Product,
                        Some("countNums") => Subtotal::CountNums,
                        Some("stdDev") => Subtotal::StdDev,
                        Some("stdDevp") => Subtotal::StdDevP,
                        Some("var") => Subtotal::Var,
                        Some("varp") => Subtotal::VarP,
                        Some(_) => return Err(PIVOT_UNREADABLE),
                    };
                    data_fields.push(DataField {
                        name: field.value(b"name").map(str::to_owned),
                        field: field.required(b"fld")?,
                        subtotal,
                    });
                    Ok(())
                })?;
            } else {
                past(stream, &mut scope, index, &element)?;
            }
        }
    }
    let (location, first_header_row, first_data_row, first_data_col) =
        location.ok_or(PIVOT_UNREADABLE)?;
    let fields = fields.unwrap_or_default();
    let count = fields.len();
    let within = |field: &FieldRef| match field {
        FieldRef::Field(at) => usize::try_from(*at).is_ok_and(|at| at < count),
        FieldRef::Values => true,
    };
    let fits = row_fields.iter().all(within)
        && column_fields.iter().all(within)
        && page_fields.iter().all(within)
        && data_fields
            .iter()
            .all(|data| usize::try_from(data.field).is_ok_and(|at| at < count));
    if !fits {
        return Err(PIVOT_UNREADABLE);
    }
    Ok(PivotTable {
        part,
        sheet,
        name,
        cache_id,
        location,
        first_header_row,
        first_data_row,
        first_data_col,
        cache,
        fields,
        row_fields,
        column_fields,
        page_fields,
        data_fields,
    })
}

/// Reads a `pivotCacheDefinition` part.
fn read_cache<R: Read>(
    stream: &mut Stream<R>,
    index: usize,
    part: Vec<u8>,
) -> Result<PivotCache, Error> {
    let mut scope = Scope::new(index);
    let root = root(stream, &mut scope, index)?;
    if !root.is(b"pivotCacheDefinition") {
        return Err(PIVOT_UNREADABLE);
    }
    let record_count = root.number(b"recordCount")?;
    let mut source = None;
    let mut fields: Option<Vec<String>> = None;
    if !root.empty {
        while let Some(element) = child(stream, &mut scope, index)? {
            if element.is(b"cacheSource") {
                if source.is_some() {
                    return Err(PIVOT_UNREADABLE);
                }
                source = Some(read_source(stream, &mut scope, index, &element)?);
            } else if element.is(b"cacheFields") {
                if fields.is_some() {
                    return Err(PIVOT_UNREADABLE);
                }
                let mut names = Vec::new();
                items(
                    stream,
                    &mut scope,
                    index,
                    &element,
                    b"cacheField",
                    |field| {
                        names.push(field.value(b"name").ok_or(PIVOT_UNREADABLE)?.to_owned());
                        Ok(())
                    },
                )?;
                fields = Some(names);
            } else {
                past(stream, &mut scope, index, &element)?;
            }
        }
    }
    Ok(PivotCache {
        part,
        source: source.ok_or(PIVOT_UNREADABLE)?,
        fields: fields.unwrap_or_default(),
        record_count,
    })
}

/// Reads a `cacheSource`, its start given to `scope`, through its end tag.
fn read_source<R: Read>(
    stream: &mut Stream<R>,
    scope: &mut Scope,
    index: usize,
    element: &Element,
) -> Result<PivotSource, Error> {
    let source = match element.value(b"type") {
        Some("worksheet") => {
            let mut found = (None, None, None);
            if element.empty {
                scope.end();
            } else {
                while let Some(child) = child(stream, scope, index)? {
                    if child.is(b"worksheetSource") {
                        let reference = match child.value(b"ref") {
                            None => None,
                            Some(reference) => {
                                Some(Area::parse(reference.as_bytes()).ok_or(PIVOT_UNREADABLE)?)
                            }
                        };
                        found = (
                            child.value(b"sheet").map(str::to_owned),
                            reference,
                            child.value(b"name").map(str::to_owned),
                        );
                    }
                    past(stream, scope, index, &child)?;
                }
            }
            let (sheet, reference, name) = found;
            return Ok(PivotSource::Worksheet {
                sheet,
                reference,
                name,
            });
        }
        Some("external") => PivotSource::External {
            connection_id: element.number(b"connectionId")?,
        },
        Some("consolidation") => PivotSource::Consolidation,
        Some("scenario") => PivotSource::Scenario,
        _ => return Err(PIVOT_UNREADABLE),
    };
    past(stream, scope, index, element)?;
    Ok(source)
}
