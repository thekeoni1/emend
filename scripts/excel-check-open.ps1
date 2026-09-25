# emend Excel check: open every row of the list through Excel
# (specs/0016-new-workbook-and-sheets.md, "The carried items" and decision 5,
# with the planning rulings during P1, 2 to 6; specs/0015-open-checks.md,
# Excel rulings 1 and 2).
#
# Runs on the machine with Excel, from the staged directory that holds it:
# the list beside it as excel-check-list.txt (scripts/excel-check-list.py),
# and every file the list names at files\<source>\<file>, the list's forward
# slashes read as backslashes. Every path is built from the script's own
# directory; the script names no other.
#
# Every row, in the list's order (corpus, fixture and input), is opened
# through COM with Excel visible and alerts on, so a repair prompt stops the
# run at that file; read-only with links not updated; and closed without
# saving. result is pass when the open returned and fail when it threw, with
# Excel's message in the message column. For a corpus row, the value at each
# non-empty reference (ref1 to ref3) on the first worksheet
# (Worksheets.Item(1): chart sheets excluded, hidden sheets included) is read
# from Value2 into seen1 to seen3: a whole-number double without decimals,
# another double in the invariant culture, an error value as the cell's text.
# A read that throws on a workbook that opened writes read-error in that seen
# column, keeps result pass and puts the exception text in message. A close
# that throws writes close-error and its text in message, keeps result, and
# the run goes on to the next row, so a results file is always written
# (specs/0016, "Planning verification and rulings on P1", ruling 3).
#
# Writes open-results.txt beside the script, tab-separated ASCII, with the
# columns source, id_or_test, label, result, seen1, seen2, seen3, message; a
# row is unique by the first three. Tabs, carriage returns and line feeds in
# any value are replaced by spaces. An open-results.txt already there stops
# the script before Excel starts.
#
# Run it with Excel closed:
#   powershell -ExecutionPolicy Bypass -File .\excel-check-open.ps1

$root = $PSScriptRoot
$listPath = Join-Path $root "excel-check-list.txt"
$outPath = Join-Path $root "open-results.txt"
if (Test-Path -LiteralPath $outPath) {
  Write-Host "open-results.txt is already there; move it away and run again"
  exit 1
}
if (-not (Test-Path -LiteralPath $listPath)) {
  Write-Host "excel-check-list.txt is not beside the script"
  exit 1
}
$list = @(Import-Csv -Delimiter "`t" -LiteralPath $listPath)
$invariant = [Globalization.CultureInfo]::InvariantCulture

function Clean([string]$text) {
  return ($text -replace "[`t`r`n]", " ")
}

function Read-Value($sheet, [string]$ref) {
  $cell = $sheet.Range($ref)
  $value = $cell.Value2
  if ($null -eq $value) { return "" }
  if ($value -is [int]) { return Clean ([string]$cell.Text) }
  if ($value -is [double]) {
    if ([math]::Floor($value) -eq $value -and [math]::Abs($value) -lt 1e15) {
      return ([long]$value).ToString($invariant)
    }
    return $value.ToString("R", $invariant)
  }
  return Clean ([string]$value)
}

$excel = New-Object -ComObject Excel.Application
$excel.Visible = $true
$excel.DisplayAlerts = $true
$out = @("source`tid_or_test`tlabel`tresult`tseen1`tseen2`tseen3`tmessage")
$n = 0
foreach ($r in $list) {
  $n += 1
  $path = Join-Path $root (Join-Path "files" (Join-Path $r.source ($r.file -replace "/", "\")))
  $result = "pass"
  $seen = @("", "", "")
  $message = ""
  $wb = $null
  try {
    $wb = $excel.Workbooks.Open($path, 0, $true)
  } catch {
    $result = "fail"
    $message = Clean $_.Exception.Message
  }
  if ($null -ne $wb) {
    if ($r.source -eq "corpus") {
      $refs = @($r.ref1, $r.ref2, $r.ref3)
      for ($i = 0; $i -lt 3; $i++) {
        if ($refs[$i]) {
          try {
            $seen[$i] = Read-Value ($wb.Worksheets.Item(1)) $refs[$i]
          } catch {
            $seen[$i] = "read-error"
            $message = Clean $_.Exception.Message
          }
        }
      }
    }
    try {
      $wb.Close($false)
    } catch {
      $message = "close-error: " + (Clean $_.Exception.Message)
    }
  }
  $out += "$($r.source)`t$($r.id_or_test)`t$($r.label)`t$result`t$($seen[0])`t$($seen[1])`t$($seen[2])`t$message"
  Write-Host "$n of $($list.Count): $($r.source) $result"
}
$excel.Quit()
$out | Set-Content -Encoding ASCII -LiteralPath $outPath
Write-Host "wrote open-results.txt: $($list.Count) rows"
