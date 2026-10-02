# M3 part 1 P2, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec, the working spec with planning's P2 rulings, specs/0001, the phase diff, the doc commit's and the code commit's diffs, gate.log, test.log, run.log, the scan's pre-check, each one's stderr, the 96 record pairs and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p1-p2. Checked commit (the phase commit): 8d05845de2f80a88dbd3b31fbd6f930410a0789f; HEAD when the check ran: 8d05845de2f80a88dbd3b31fbd6f930410a0789f. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to b5943779c93173354a7e78bdf565c08bc5a92548f74235af3dd238eb1b8866b4. Its report-wording items were corrected in the report; its log and record items are reported to planning as findings, since a corrected run.log would mean repeating the runs. No second check was run.

Non-ASCII replacements (specs/0001, T1 P2 departure 12): the answer held U+00D7 MULTIPLICATION SIGN at three positions, characters 4998, 5018 and 5043 of the extracted answer (0-based, in the line recounting tier c's outcomes); each is written here as the letter x. The extracted original is kept at <phase>/check1-answer.raw (sha256 in the paragraph above); nothing else in the answer is changed.

Inputs and their sha256 (the 192 record files listed by name; each is also committed under records/):

- 795e12648d2615b93d40011cd3e264808a1342d1054df742d1c8e9fcd0f76cab code-commit-1.diff
- a9fafe767e6ee4db8c07d4aa14c351f7dd14ee7045db556aa847c2ae72a7e335 doc-commit.diff
- 583e9ee95f34dc6de51693ff59b1a83c2d9c9614baebc89382b1e5d184365e45 gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- cc3160ee0fb30c3eb58c96c670a577cca4a2259e57cebc15f6c927863b85d1ad phase.diff
- 9ea459117cecf9cd995256bf815c494acfe59ee7a4f74ceb35bf1d79de147f09 records/m3p1-scale-a-border-2026-10-01-8d05845.md
- 7e91e1d1bd7186e14abacfd3672281f42df3ebde051c50e25c93befe3f3cb16e records/m3p1-scale-a-border-2026-10-01-8d05845.txt
- e7a86b437f1a3997ee616f473451be16204430b2cb5a7ee3590915717dff52c1 records/m3p1-scale-a-cf-2026-10-01-8d05845.md
- 7b9be64c66b641dec9cba291dea45e818c56478acf2a09d9fa1c5a6b522be91d records/m3p1-scale-a-cf-2026-10-01-8d05845.txt
- cd508fcff78da4af6223754d2dfb820e22af274ac35f3c69491f7826a7f305a7 records/m3p1-scale-a-comment-2026-10-01-8d05845.md
- 2669d9b55a39d33dac0d0a3f8fab71c87adc53886e7ae89c28e300e6a1b42bcc records/m3p1-scale-a-comment-2026-10-01-8d05845.txt
- 1b88ec522fb41bf13c2b0d416eda34ca6b497d505a605ac7a04095f07831095e records/m3p1-scale-a-date-2026-10-01-8d05845.md
- 0b513e534c566d68a6ffd93f5053fc68f632d7f0846c94a6ede04acd4a11c206 records/m3p1-scale-a-date-2026-10-01-8d05845.txt
- 9bc7051c9fa2288c75c7485cee9ac0fe073e9370cfdf2e1f049ac2d71225c457 records/m3p1-scale-a-delete-columns-2026-10-01-8d05845.md
- 9724e94c7ccb8c6d2e8c3e4f087074fbe9d66bb5d1a5d08755ab3d4182bfc516 records/m3p1-scale-a-delete-columns-2026-10-01-8d05845.txt
- f13be553429e18fe2434ccceed5fab4ea3f9e49bb0fa746cd48d6189933cc0c1 records/m3p1-scale-a-delete-rows-2026-10-01-8d05845.md
- 53ee2062aedeefbc4686e83c892c978b8c6c9721601f4db0200b30367ec1f2af records/m3p1-scale-a-delete-rows-2026-10-01-8d05845.txt
- d28c726737cb6d59f5dd41507209ced327225c8de84219f23e7d6e91a2585649 records/m3p1-scale-a-edit-2026-10-01-8d05845.md
- a40a6138429a576a24066f7e0e3f7a8bd8d2539f15a049438caeca03657a14f8 records/m3p1-scale-a-edit-2026-10-01-8d05845.txt
- 3e5c0ee4b9da90e28d2becde16b4f39dc60cd232b68a86c8186b9a17283bb273 records/m3p1-scale-a-edit-formula-2026-10-01-8d05845.md
- 91a9fb62dab450aeee1f50c4caf5e3acef56aa4d265c212b33531c38308132fb records/m3p1-scale-a-edit-formula-2026-10-01-8d05845.txt
- 141d97747e1d43cfd5dbd89178fdf9d7ab45a81a44abd662ecb5fe9dfaed8c70 records/m3p1-scale-a-font-2026-10-01-8d05845.md
- 8ca9b7943efc5cc50d6a03be4b3b6ac4fabe45959f9864756d144820e3543817 records/m3p1-scale-a-font-2026-10-01-8d05845.txt
- 261bbc0ddd3280bf1f7eba726d5852bbc2db17ea903c627d09858ec19e5c7f96 records/m3p1-scale-a-format-2026-10-01-8d05845.md
- 5e59411c5d891ac58ca7a67d813649b2dec82c8a9c014441bc4e284151242e6e records/m3p1-scale-a-format-2026-10-01-8d05845.txt
- 1c49f23a978716143b38d9616cdec975882a5f7d1b78ccd9352518263ae20bcd records/m3p1-scale-a-formula-2026-10-01-8d05845.md
- 1659374cd444c7e0311860f5fa4616fa3a80dc9b6e52c0504d2a56b5c6b088c6 records/m3p1-scale-a-formula-2026-10-01-8d05845.txt
- c981ccd5110fd2d1bb0d2531aa3bb18b6e740010c439d256440f2b85f6d4c92c records/m3p1-scale-a-image-2026-10-01-8d05845.md
- 28dfb069b2df9c36fbc144f501eebc1629008bb305623f4d427dd46c44c5e390 records/m3p1-scale-a-image-2026-10-01-8d05845.txt
- 7061c3f84869c8d234cecc06ecc1787ffe6136e9b52a8e028379f75bfbbe44c8 records/m3p1-scale-a-insert-columns-2026-10-01-8d05845.md
- 792787718e1f5f6f0809eac34bb246415d110bdd03d18ace2fc4e366f1f5ea7a records/m3p1-scale-a-insert-columns-2026-10-01-8d05845.txt
- 375a21b94c6b5d019bf330090b11d1b8517c0cd0f913a74c255c2fbc4353da96 records/m3p1-scale-a-insert-rows-2026-10-01-8d05845.md
- bd8b78941a1bc1d629af48279430a88453c36adf3ce0d4682b4961a276b9621c records/m3p1-scale-a-insert-rows-2026-10-01-8d05845.txt
- 3fa9920314fe87c022fecac8e5fdc5f0c9ed290178ef57a47ed2c15e4d78e46e records/m3p1-scale-a-link-2026-10-01-8d05845.md
- 11224dd9665ff58313b73565500c6d9ecc548f1eec9f50e9e050db59bb59474c records/m3p1-scale-a-link-2026-10-01-8d05845.txt
- 1d33edc9c7a41c3aef22e3b51687944ef30b3239d4faada57d7db0f4c496c102 records/m3p1-scale-a-names-2026-10-01-8d05845.md
- fc545d3597c0b526082949c6a58ec287e6b839241970ed7b5a6ef1dea88577b3 records/m3p1-scale-a-names-2026-10-01-8d05845.txt
- d5a226182c6d46a2ec9f19889a11600fa89395759137741f48d6ebd11ceeeb73 records/m3p1-scale-a-props-2026-10-01-8d05845.md
- 17cea3e72745dcc18bb442407f77354e4f5826f084c834654a78160de5b6da80 records/m3p1-scale-a-props-2026-10-01-8d05845.txt
- 220dae3bd03311c2bb5a30e28aa71af7062344889069c4044f310b964021ad48 records/m3p1-scale-a-protect-2026-10-01-8d05845.md
- 156c0ac6aae9c1c8b0f06bc016f2e0f75ecc58c650f65fec062eecf0b24d7ccf records/m3p1-scale-a-protect-2026-10-01-8d05845.txt
- bb355c6d46131be9a8e82189099e9cd260f9061ed481c8befb13e425d5d45114 records/m3p1-scale-a-remove-2026-10-01-8d05845.md
- 9844c7423f3d0a6b36a73fe39cd7fd62625d87fd0c9655956a182a1dc575c8c5 records/m3p1-scale-a-remove-2026-10-01-8d05845.txt
- 226c195af68e5aa657920665d788fe0fa9d6a40add84dd11a4c11eb730b48dcc records/m3p1-scale-a-scale-2026-10-01-8d05845.md
- c89064327135b161bc56dafedf3c19c7c341548141712f96428d3829d82e0c9b records/m3p1-scale-a-scale-2026-10-01-8d05845.txt
- 430390c6e5cd5b4aa44b788696c276b573d426b13a2a6f74cac148ba52d7f96d records/m3p1-scale-a-sheet-2026-10-01-8d05845.md
- 8e9251cc3ddaa57e112f51e964af3e07d43536397c99f309a6ce555375ac1778 records/m3p1-scale-a-sheet-2026-10-01-8d05845.txt
- c98f4cb342943eee14fbcc9dd1f58695189d956cf85cc868f928746b7f02daa9 records/m3p1-scale-a-style-2026-10-01-8d05845.md
- cc1a865402f042d293d7fc6829826999930b364672ea2351c433f156ff297a04 records/m3p1-scale-a-style-2026-10-01-8d05845.txt
- 9c58743c2c792a5783ecda254e1c27d3ef946ad9b3291ceeff1ce9c8d26cdff8 records/m3p1-scale-a-table-2026-10-01-8d05845.md
- 38ddc37684fe6cb28a46b619055e8da4243fa81e13aef2c88c37415da86001c8 records/m3p1-scale-a-table-2026-10-01-8d05845.txt
- 9255141662a6589fb9ac04ac0f7779c0fbedafdf37ba5a19b2846c0a622d57de records/m3p1-scale-a-validate-2026-10-01-8d05845.md
- f42b50d4772c541471290fabd911781e6e33428f501b7c7961497e20bbedca3e records/m3p1-scale-a-validate-2026-10-01-8d05845.txt
- 8ff9d807ddb2d596535a4625585b0815664f4a179476934e633d1c856490deb2 records/m3p1-scale-b-border-2026-10-01-8d05845.md
- 627d35be6149bc6e72e623586aea68c1dc0ca5d5592145f41500792f356c6bcf records/m3p1-scale-b-border-2026-10-01-8d05845.txt
- 11e9c810e93715b0c9fc0410a7c7fc7d5e514b3a0bb12ab814fcd36fa8e97fe8 records/m3p1-scale-b-cf-2026-10-01-8d05845.md
- 8bc6b297b015958b99adb586220ee36f14892d8366823920e80d1a16e7319400 records/m3p1-scale-b-cf-2026-10-01-8d05845.txt
- e7c213be8fc101d5a70fea7072f5dfe39b7fc556e2a91b6982a20a3ac62c7a77 records/m3p1-scale-b-comment-2026-10-01-8d05845.md
- 4fea5115d8545347f244427470fb2401c59959a75659aac38f22921eb7fbffe6 records/m3p1-scale-b-comment-2026-10-01-8d05845.txt
- 934d4319f4bd480c432c0a1ec9262d318b903fa077542eba8958421c0a42930b records/m3p1-scale-b-date-2026-10-01-8d05845.md
- 6b008fe68db01c2ca202a9c7a3267ca5a2a18b7507261eae3ce6e371e61fc1ef records/m3p1-scale-b-date-2026-10-01-8d05845.txt
- 5757e08834a0bdd378b35719b48d69ecb724114918861b6668d12d0bc916157d records/m3p1-scale-b-delete-columns-2026-10-01-8d05845.md
- 7f5f57fc3bf76c2f6e332ca96b73ffb6077971ae964ac4d5066c8c76baa2c80b records/m3p1-scale-b-delete-columns-2026-10-01-8d05845.txt
- a584e0ba9e42cc63260b2ad55c6574350afb62a4d463d4c1fb02642dd86581e1 records/m3p1-scale-b-delete-rows-2026-10-01-8d05845.md
- 704e5114c72fc8aadcb71f4a706d094cca72679e4879f7787f78fcbc60dc2816 records/m3p1-scale-b-delete-rows-2026-10-01-8d05845.txt
- a4236260a148639807bb3677151864416f393a7e04cca5dd7e79e127538558ea records/m3p1-scale-b-edit-2026-10-01-8d05845.md
- 81281caefe9f831826945a1adc79a2f06f9c8d53ba2bb7fdc05058ad864e7ed3 records/m3p1-scale-b-edit-2026-10-01-8d05845.txt
- 7599f80363335ba020e347114b25cd778f300ed37aa6010cec2dd15575d684df records/m3p1-scale-b-edit-formula-2026-10-01-8d05845.md
- 2b16bb6c2de1b9d940500185ac60faf719547bc55b7fe5a999d6a9e7aef46160 records/m3p1-scale-b-edit-formula-2026-10-01-8d05845.txt
- f40390ef3787408cf7889f1fb8449c14b7edf0ef5cede1ee3e2a0c8492a059c8 records/m3p1-scale-b-font-2026-10-01-8d05845.md
- 7f4998dd83654c801e66b328d45e9eb7eabe0fc832bdb4fa76a52e685ce3c9a6 records/m3p1-scale-b-font-2026-10-01-8d05845.txt
- f2419c69e9ae17fba5e72ba030a2329a5a5132c2d7d132b62ea1ec83b20708ae records/m3p1-scale-b-format-2026-10-01-8d05845.md
- 347c656fb2d1e68c8dd2152d70cfef3e95a7894289b68f960b1879e056d8eea4 records/m3p1-scale-b-format-2026-10-01-8d05845.txt
- 5237ca1c288432c0013f57cacdd8c9774016f5f0f09894bba61767b0fad1b91a records/m3p1-scale-b-formula-2026-10-01-8d05845.md
- 1cec55c162b2e07dd39a29519b81fed0e535e2337f923df358b5e046a05d438f records/m3p1-scale-b-formula-2026-10-01-8d05845.txt
- 89921995317a5e8e41c8e0f6ca997519cc57cec24288c28c323e800c336a50c9 records/m3p1-scale-b-image-2026-10-01-8d05845.md
- 48a37816f02859c67cc518d87b920df407fdf87db2d022a22538dfe61c3eec21 records/m3p1-scale-b-image-2026-10-01-8d05845.txt
- 5f6bcda15fb943e96f3bc851d72a6878c2a42aa9f2327655fd093490b6e87bac records/m3p1-scale-b-insert-columns-2026-10-01-8d05845.md
- 5275e314fda47e0656a1caadb9c97b5eda7eca233dc853f859087f389f1c2816 records/m3p1-scale-b-insert-columns-2026-10-01-8d05845.txt
- 9261868f3d48a60717329d4e919847b2c1cee153a1a923ce6b10f8e0d33bf78a records/m3p1-scale-b-insert-rows-2026-10-01-8d05845.md
- e1a6445e69566fa8f79bed6e4bcc6ad10fc38ecc925784b206637a740410e0d6 records/m3p1-scale-b-insert-rows-2026-10-01-8d05845.txt
- 22706dc07f180a8de3a663b7908e4c7d2da04285acfd396293cfb3ca1b845332 records/m3p1-scale-b-link-2026-10-01-8d05845.md
- 738e72c977fb32c3424121e7d4ff69d54fa648dd7b0b5749ba79baafabe2b10f records/m3p1-scale-b-link-2026-10-01-8d05845.txt
- 97f496d4923be514ced5f9bcee7ed1a3c87193d3be0c8050405eab7e1320409d records/m3p1-scale-b-names-2026-10-01-8d05845.md
- 13233ab9e948c9d44bcd86ab32f0331a755eddbc2138656ced1305c806017d48 records/m3p1-scale-b-names-2026-10-01-8d05845.txt
- 196ea5c9cd0e7533ff77c34df3598819a927a27a9b50f1d9da521c919a6c927c records/m3p1-scale-b-props-2026-10-01-8d05845.md
- 0f6b6a4ea62301dcdcd97c7ca96d25c724ecb902e993838850b672b078096441 records/m3p1-scale-b-props-2026-10-01-8d05845.txt
- aac15bb498986d180b83b3f530c38979e7c8c0fc283f813777d16965b5c261e2 records/m3p1-scale-b-protect-2026-10-01-8d05845.md
- abacd5e1e1db8f5fda1e92d3c5cc0b04a9604d64c016fa41aba797544516c4dc records/m3p1-scale-b-protect-2026-10-01-8d05845.txt
- 4388b40d3c70859634fefb3fe2b6919b28c516cb0dab8514b60d49ac4ad36817 records/m3p1-scale-b-remove-2026-10-01-8d05845.md
- 388902332ecac1b5f1046bcab3f4146b85f9f75bd725f586329f509baec66e03 records/m3p1-scale-b-remove-2026-10-01-8d05845.txt
- 13790bbee95822677fc9130f0a7eb9a41ceb21d0379da2f34ac4d0dbad279a12 records/m3p1-scale-b-scale-2026-10-01-8d05845.md
- 4b09c91d0df13671abdcf3197142ec49f6ae50fdc4ebd880ea3b0d2cfe150440 records/m3p1-scale-b-scale-2026-10-01-8d05845.txt
- 101318996e3b0063ade19a1d479d7706960f8ab6c982da0819b5dee04598e1a6 records/m3p1-scale-b-sheet-2026-10-01-8d05845.md
- 3e29fb522ada83a6330e36f6f55de28e0cbeb740c15489dfc009af54f4ef47da records/m3p1-scale-b-sheet-2026-10-01-8d05845.txt
- 7fb2a2cc138f1865317bec1ad191ab9e22e7d6cd3cbd3b18b5654ec068056ba7 records/m3p1-scale-b-style-2026-10-01-8d05845.md
- 296f71fcded94272c8aad83a2236838f1df073cb3ce4aa3019c38e0f57bf3f06 records/m3p1-scale-b-style-2026-10-01-8d05845.txt
- 258ff77d4f76327935277754869735e907541f32d67a4b233d620d3aa6f0b62f records/m3p1-scale-b-table-2026-10-01-8d05845.md
- 03b2cd426bffc40dffa8ca99610688aa346215573b10955735d1e46f070da479 records/m3p1-scale-b-table-2026-10-01-8d05845.txt
- 0cb626a5a38b868f117231350a7cff360ed914b0ac4a81d914fa0ed0b346f61f records/m3p1-scale-b-validate-2026-10-01-8d05845.md
- 9fe1e14f893c8a92fbba43b68f84d45d9db974f9ce9af00ead055f5781f4e267 records/m3p1-scale-b-validate-2026-10-01-8d05845.txt
- a297616eec9e16b5fa39b60db044186deaa4d5e6bc509caa3f872bd64c482d9b records/m3p1-scale-c-border-2026-10-01-8d05845.md
- 21046439cacb47f9c4a72c04eb1564c1d7a228d3394319157996b7588b4c9a89 records/m3p1-scale-c-border-2026-10-01-8d05845.txt
- 0b36d5c5a6ca098fa8ba8bd270d07945637f3fb817cbf95421373c7544d5edfc records/m3p1-scale-c-cf-2026-10-01-8d05845.md
- 16ba39cdf95164bc60d12de225bd908f562e21544b45b1309a4b99e69fa19929 records/m3p1-scale-c-cf-2026-10-01-8d05845.txt
- 9abfd77fe5e90dedeb7cac5da77ed30890dcc2c9e217fb496b3391e889a9f4d9 records/m3p1-scale-c-comment-2026-10-01-8d05845.md
- 7c457377180e741396f1a3eb92c4c041475c89369b848601f9e6ddf274fe7221 records/m3p1-scale-c-comment-2026-10-01-8d05845.txt
- f56c93278eb83c8a80b6b218445ea85c7eb3ddf640dd4682b43609eb1abefb4f records/m3p1-scale-c-date-2026-10-01-8d05845.md
- 7e76929c41581256ff5f2b146b2f7207ae9640db5c377f3d86a2427a836b016c records/m3p1-scale-c-date-2026-10-01-8d05845.txt
- 2d924b7a2a9e0d3bb9d056385ab3bda1a3604e36867af4094582cac19a29ddcd records/m3p1-scale-c-delete-columns-2026-10-01-8d05845.md
- 4321ac8276366a733199d8dd4e0bbafd19bfeffb61a58bac3471d103d9328d53 records/m3p1-scale-c-delete-columns-2026-10-01-8d05845.txt
- 9e87611ae7018013ca582aba0b8d0abae0f28e22b26e909974bc53898f3e420e records/m3p1-scale-c-delete-rows-2026-10-01-8d05845.md
- a87a8ecb59e4863c2f2e8e70cc3d28f3ff8abac26811a9da648334a8230e6d73 records/m3p1-scale-c-delete-rows-2026-10-01-8d05845.txt
- 623b16a68d974b54c43f59cb9d90dab0d2e28ced550c051a751bf276c2aef89e records/m3p1-scale-c-edit-2026-10-01-8d05845.md
- 1219955556d32f4ed0bf788eaaa70e665bf35767049ed2227fd2f3f9e0b0ad08 records/m3p1-scale-c-edit-2026-10-01-8d05845.txt
- 4cb4cadcdc7c9047067cc232c0cc88a942588d46775275334f0cc44db527bff4 records/m3p1-scale-c-edit-formula-2026-10-01-8d05845.md
- 4e351d609b5f8a91f836d7cd390a7b0670bdf7636fd19fa5d840f5ee80d19958 records/m3p1-scale-c-edit-formula-2026-10-01-8d05845.txt
- dfcde9b8068878356139ce6ea3a99827ca20d902b15790acec8ee0db0789113d records/m3p1-scale-c-font-2026-10-01-8d05845.md
- f1bb14a9dab51c59e33f7a7f8f8b9531bc416482567662b6515bf9815534b3b7 records/m3p1-scale-c-font-2026-10-01-8d05845.txt
- 00e78d13de5e566b58ccf05e837a1c9bc2eb39dc75fe2b57936a756557a239b5 records/m3p1-scale-c-format-2026-10-01-8d05845.md
- 1ad01ef4b24d55ad2c8d23954d84b9bb13e0502def1b4d0d509f6a9810bfa079 records/m3p1-scale-c-format-2026-10-01-8d05845.txt
- f6ce452985f28d23f9d4c4056c7e08647afc5783872c84279cf54e87c0a6bb24 records/m3p1-scale-c-formula-2026-10-01-8d05845.md
- 4e60c5faacd7c9bfd486693859d5492f093cd5b711f343d16bf6b10ac7d535b0 records/m3p1-scale-c-formula-2026-10-01-8d05845.txt
- 0573be78ac47956a828656617f993ca02cbb977653194cc2dc2318bb134827a9 records/m3p1-scale-c-image-2026-10-01-8d05845.md
- b1f2209e0767833495b3c2db98a04f4293ceb93bf9a908d364376206bd20399d records/m3p1-scale-c-image-2026-10-01-8d05845.txt
- e2af6e40889ac537711a80194fb9fc3d04aa7bdf70662c08d27034741ca517a3 records/m3p1-scale-c-insert-columns-2026-10-01-8d05845.md
- 4f45ccdb594c2ea05730ab7dec35a32851f781e60ca37ea94645cb311a74c74c records/m3p1-scale-c-insert-columns-2026-10-01-8d05845.txt
- 42c52436bf1f261513aa849c3cd4a5b6a18b9b7a454f0c6791fc589cf615aa9a records/m3p1-scale-c-insert-rows-2026-10-01-8d05845.md
- 885a816cec0ce9b3d0232509b2a947fec2fa0fde4889ff5ce0322ffae230219b records/m3p1-scale-c-insert-rows-2026-10-01-8d05845.txt
- 6258bc9ca402898ac54ffbd63a60cc78b8b290dc157bd826088159b07ab2f4f7 records/m3p1-scale-c-link-2026-10-01-8d05845.md
- b46ffac243b02a812d58e302f13c181246146dd709f808ea3cabf4c5188d9715 records/m3p1-scale-c-link-2026-10-01-8d05845.txt
- c5face73e12c7d90e3d9471703eb80f243fdf7385ef9ce970d6831cfff0b94d6 records/m3p1-scale-c-names-2026-10-01-8d05845.md
- 144bc092c8ae2cdb551495b2faca2f90bbcb47edda40656d2355c35fa0c00d75 records/m3p1-scale-c-names-2026-10-01-8d05845.txt
- 469b80248b651f487a278c4c64fd0a11f85af4ff540dfc1f86ce2240fe58497c records/m3p1-scale-c-props-2026-10-01-8d05845.md
- 62a5d7702dba21c2201e88a21c394f843f263d55832d23ac83149f9ee8a7bbd0 records/m3p1-scale-c-props-2026-10-01-8d05845.txt
- 2ad2d36d0a2f68dcf806069791c3fc529d6eb2aa682d9d80fb6d0cbf742ec734 records/m3p1-scale-c-protect-2026-10-01-8d05845.md
- 38ac013bd7a474add33cf43fb671d99af722e535e885afdc2553fc76b30bea55 records/m3p1-scale-c-protect-2026-10-01-8d05845.txt
- e5fb8dbdba6bc5c603851934c19e2a922ee3d83fb3254564a72f64991abda4a8 records/m3p1-scale-c-remove-2026-10-01-8d05845.md
- 1ff0f6d142e503e7b2bd0cbeb433d86108ab390edec9b9a060ebd6524ba5e258 records/m3p1-scale-c-remove-2026-10-01-8d05845.txt
- 4c0856a397904a508ceb575f5fcb468c08e16dee02cdd870c5e1c2cfd7ed901b records/m3p1-scale-c-scale-2026-10-01-8d05845.md
- b96059d796cb1b9602c4578ea6ab7c0de41eb2ae35d30c9377f7abf628e9f231 records/m3p1-scale-c-scale-2026-10-01-8d05845.txt
- 67cf84fb292dcf0fff891b5007d52e10ae6d3d51ba289aa3efff8b47ab414854 records/m3p1-scale-c-sheet-2026-10-01-8d05845.md
- c876e0c9cbf8e79e6bb27de524ae939fc2500f6a751239a0870822228c68b13e records/m3p1-scale-c-sheet-2026-10-01-8d05845.txt
- dc00aeb1d07193a148a1fd94b7f923dca918b034873d3661f997d7852679e226 records/m3p1-scale-c-style-2026-10-01-8d05845.md
- 683e8061fa854d04a25e53217ac59da902988e2ccb3d006ef503f0e0c10855c3 records/m3p1-scale-c-style-2026-10-01-8d05845.txt
- 60bcf1c12d49ad31a8a0b05f8e91c935afe81b5e9699f08495685aa7dd7ad6b6 records/m3p1-scale-c-table-2026-10-01-8d05845.md
- d62ed99ab165bbd180541732b4d24dc424305bd35be4b81650d11d6584d1c11a records/m3p1-scale-c-table-2026-10-01-8d05845.txt
- 4f658835498b62124b3a7b4874ce90b5887b4d73560848fb70e87c205724aeab records/m3p1-scale-c-validate-2026-10-01-8d05845.md
- 9d45a5fdf569be2689ce34b94b044aef0221a08e9fbf101b6059d8aa48049e1f records/m3p1-scale-c-validate-2026-10-01-8d05845.txt
- 82d8048b2c328cde211752fb499f7fabbd167cdcf48d8114ec4767ab256d4b13 records/m3p1-scale-d-border-2026-10-01-8d05845.md
- b8897c0c9b5e79a9039d48f877c0b2a7c0b03f10d365893bad3a81ed405d3008 records/m3p1-scale-d-border-2026-10-01-8d05845.txt
- 798b404648a5cd1fe12ec100ce1f8126e531377c7c75acbcd9091f39021bc0b2 records/m3p1-scale-d-cf-2026-10-01-8d05845.md
- db399dd24097aa87c0e10ffaa8f3b9d17f27d47bd490f3475b72057a3801c347 records/m3p1-scale-d-cf-2026-10-01-8d05845.txt
- a50adf5f04aaf378249a402c716379c51e3dd9b88b10ee73cbd7a6a6513e2839 records/m3p1-scale-d-comment-2026-10-01-8d05845.md
- bf7b0ebf59c6f81aa02ef07bbab68d1a6f488e216e10f30c831e1fcc799f0b7b records/m3p1-scale-d-comment-2026-10-01-8d05845.txt
- ecf3c86b0d8aa293cbbb8496c263b7e8b1e458d89a48b9c013ed58746d5a7324 records/m3p1-scale-d-date-2026-10-01-8d05845.md
- 7fcc6d05cfaf0c74c30de7048367c2e64be73f236e4ee59d2a002f84c4bff488 records/m3p1-scale-d-date-2026-10-01-8d05845.txt
- 39b01ddc6c3f3b133436933641ff917b1cccd2151e66794426274aad1a43e74e records/m3p1-scale-d-delete-columns-2026-10-01-8d05845.md
- e6b29deb2dd06190b00c3678b398bfcbbb02ad9e6c2fdf8b92b685bf0ec668d1 records/m3p1-scale-d-delete-columns-2026-10-01-8d05845.txt
- 770bfd884ff85deb5ab2582299e063bce3e2923b44347a2adb56e8613ea26da2 records/m3p1-scale-d-delete-rows-2026-10-01-8d05845.md
- 42fdbfdf3f2a2d9a52e7cf645b9f32749fab73aaf112e4d4a72204e850d0028b records/m3p1-scale-d-delete-rows-2026-10-01-8d05845.txt
- 0db6ffe4a9134427708ee0ae033ce90a590eabebfd04677a0f942f9a5c068e98 records/m3p1-scale-d-edit-2026-10-01-8d05845.md
- 6afb7e541aa3df5880c9a5045111f56ce0675c3ea72758923c8b81c168776355 records/m3p1-scale-d-edit-2026-10-01-8d05845.txt
- 9220d432fcecf7db71be906e11ce5e4a79c851319306a52c138021542f7378ee records/m3p1-scale-d-edit-formula-2026-10-01-8d05845.md
- 07dce30a5bdb8f3d7558f5afcdc9d33c7c2643a7ca931c91e306d4f6dd27cef9 records/m3p1-scale-d-edit-formula-2026-10-01-8d05845.txt
- 4b15f3d47299985aaaed20ba8adb5fc8bf97a980024fa17404e598324b002713 records/m3p1-scale-d-font-2026-10-01-8d05845.md
- 717b2c34f03aec1c7688d54b82f22a702a4106049ac07d3bdf6f385b9188194e records/m3p1-scale-d-font-2026-10-01-8d05845.txt
- 48dfebd9cdae7806ce3de8e5f235f59ccdf01f81c27030cfb7c7a7daf3c80ab6 records/m3p1-scale-d-format-2026-10-01-8d05845.md
- 59668fe57d71cf2ebf6366d8d6923bf1994d0f1d2d31221b66511ce3c33bda43 records/m3p1-scale-d-format-2026-10-01-8d05845.txt
- 9d7b27c5a67d6fbcfc338647b7c2ce3e4813184a3739faabf942fc2dd6e89dd7 records/m3p1-scale-d-formula-2026-10-01-8d05845.md
- fc62aa59b4eabfd2388957295be1cfca8d4f24ba482f369c46973bcbade0c27b records/m3p1-scale-d-formula-2026-10-01-8d05845.txt
- dd5fe39ab8a96109be9f47bb11c14db3d7205e4f0c785fecd52e6be0ce6b6023 records/m3p1-scale-d-image-2026-10-01-8d05845.md
- 5e70d601e559ea651c9209bdb87bf9e1375d4f980697db14adbbee4d8bdb5e8a records/m3p1-scale-d-image-2026-10-01-8d05845.txt
- df5faa1ea8df7d8bbd007d5916d7f5828b502936770f082fa0793a5232febdbc records/m3p1-scale-d-insert-columns-2026-10-01-8d05845.md
- 1441c8ac8c5a412bd1651dbb09faf4f291cebb9fadcffacf67e58fc8e06f90a6 records/m3p1-scale-d-insert-columns-2026-10-01-8d05845.txt
- 864a31cf0f32705676529de1e23bc3eebf3d6ef93104958d9619ac5d56299780 records/m3p1-scale-d-insert-rows-2026-10-01-8d05845.md
- 65b03561f1908199b7a318e5e27248b0295a5d00c209f62eb9071303b3758d05 records/m3p1-scale-d-insert-rows-2026-10-01-8d05845.txt
- b2d1b6b4b06862630438169ff2682c2fb226bf10dcd151db17636f2f629541c3 records/m3p1-scale-d-link-2026-10-01-8d05845.md
- 8f9b758cc9b632fa9c4285702ceec78b6d5de3f98bf6a57c842eb7811798326f records/m3p1-scale-d-link-2026-10-01-8d05845.txt
- dcc3c28ed3f49300b60b9959e0671d784fa8f64d22d658369e529a37ccaacacc records/m3p1-scale-d-names-2026-10-01-8d05845.md
- e02420cc66e9fb37f88be477e01cdc57fd0fa0239496f629f57c6f3aeb36dc59 records/m3p1-scale-d-names-2026-10-01-8d05845.txt
- 9dfca448da154cc5582160b0fc63c4fa1466870c658cf4d5d6481c42f140fee7 records/m3p1-scale-d-props-2026-10-01-8d05845.md
- 0b293c97e572f92cee2f2884c5bd4e4e3ea693045a95d934d6ad92457e4c27b2 records/m3p1-scale-d-props-2026-10-01-8d05845.txt
- a01edbc888bff3eaf57af1d13ece351dcb50d9faabdfa8ec8e7748ff90181eab records/m3p1-scale-d-protect-2026-10-01-8d05845.md
- 14e80b73efc0ead16d92b6d02bf349844b910ffef36cdf76ce56173dd361e613 records/m3p1-scale-d-protect-2026-10-01-8d05845.txt
- c5c6c80cb9536af15421ec9a02dd7c445a66ebab1ab7f482643ac3115b7a1e04 records/m3p1-scale-d-remove-2026-10-01-8d05845.md
- d4bcbab3273e40d5964d1f4bb038d8f4786613890b74fb7d40f621db4da9801c records/m3p1-scale-d-remove-2026-10-01-8d05845.txt
- 196ee285ca9395946f62c57399059c540db35b7d8a8512e80e81a4b0c6af810a records/m3p1-scale-d-scale-2026-10-01-8d05845.md
- c96c58a3094cb1c78d0bcc8bec6f220ad733292536747a56290dd1fda77f6d69 records/m3p1-scale-d-scale-2026-10-01-8d05845.txt
- ff715342ccd773e3705c89d5174e343946bf9cf93f82c7683834226255f1686a records/m3p1-scale-d-sheet-2026-10-01-8d05845.md
- 056bee1abdca66dcffe562989084366b2ee036dafc61d9090c7ece5919ed6fad records/m3p1-scale-d-sheet-2026-10-01-8d05845.txt
- 43db51df11021efc78a0a6af7dca5710d9bc88ee6a04ef1e68bbc23335d9a934 records/m3p1-scale-d-style-2026-10-01-8d05845.md
- 7eaca2691920850dfa3361fc7035ea9040d915416c44bd9376d26744293e996e records/m3p1-scale-d-style-2026-10-01-8d05845.txt
- 5a17afb8c3a37aa9b2b28ed2e49be065e951b3b92ab602690eb9c9342ee12f38 records/m3p1-scale-d-table-2026-10-01-8d05845.md
- ec0cd579e3da5dddce4af9c8ecad39989edc570edbc4d539026e4e78cdff9cf2 records/m3p1-scale-d-table-2026-10-01-8d05845.txt
- 556602001bf48d8f8f816d59ab5493cfeb053b34dce90e9838f782053e22288a records/m3p1-scale-d-validate-2026-10-01-8d05845.md
- d39a2b50363c5f8db64bb37124e8fc9a1f9095084199decd16dc012398d05787 records/m3p1-scale-d-validate-2026-10-01-8d05845.txt
- 362806c14c810f66b98be91f0ac1218c92fb07993329a1f7315e9620d4b63989 report-draft.md
- fabe1f70cdf6074c72e5d5254f0c2ebd62e664376ad6b803908ed5f1b0ebc63d run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- d78203519084f80083ec969edc1316bb99299539aea89b936ae1625004d2e8d6 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 30b61067788c5c48155a82e4cd7fd87549d23cff139a367a68f00630266db4f3 spec-committed-0023.md
- 858445d1811493193c299c38591e031f79610ef360407cf23a2a2b4c706291ab spec-working-0023.md
- 07ad5b0958651b7da70b0cde88b4f9e933ced8a83c9f9c8608af1aee3bf37862 test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

---

I ran this check from the files in <K> only. The harness put a CLAUDE.md, an auto-memory index, the git status and a commit-attribution reminder into my prompt. I did not use any of them as evidence. Besides <K>, I used read-only git in <repo> to confirm commit ids and hashes and to read source at 8d05845. I also read <repo>/scripts/passthrough-run.sh in the working tree; run.log line 1022 shows that copy matches the phase commit's.

**Q1. Does the diff do what the spec says for P2, and nothing else? YES**

- **code-commit-1.diff** matches `git diff 38ec8c2 8d05845` exactly. It touches only crates/emend/src/package.rs and package/tests.rs: 84 lines added, 6 removed.
  - `Parts::validate` (fn at line 8087 at 8d05845) gains one check. It refuses any `cell_edits` or `style_edits` entry where `!in_grid(at)`, returning `INDEX_OUT_OF_RANGE`.
  - `in_grid` (line 8986) is 1..=1048576 by 1..=16384. That matches ruling 5 with P2 ruling 2's renaming.
  - Clears are covered because they are recorded in `cell_edits`. The test exercises all three edit kinds.
  - The rest of the code diff is the catalogue sentence and doc comments on `set_cell`, `clear_cell` and `set_cell_style`.
  - The test covers past-row, past-column, row 0 and column 0, and writes the grid's last cell.
  - Nothing outside ruling 5 is changed in code, and tools/ is untouched.
- **doc-commit.diff** matches `git diff e2ff972 38ec8c2`. It changes ROADMAP.md and specs/0023 only. Their content sha256 are 75fe5ce7... and 30b61067..., which agree with the commit message and the report.
- **phase.diff** matches `git diff e2ff972 8d05845`, which is the union of the two diffs above, and nothing more.
- Both commit messages are ASCII and end with the Co-Authored-By line only.

**Q2. Do the logs contain every line the spec requires? PARTLY**

Present in the logs:
- **Spec pin:** 30b61067 in gate.log (lines 21-27), test.log (lines 22-28), run.log (lines 1027-1031) and scan-pre-check.log (lines 49-62). Each has a refusing control, and the working-tree copy 858445d1 is printed as not the pin.
- **Phase commit vs HEAD:** run.log lines 36-39 cover full sha, HEAD, the null control and the short-sha control. Commit roles are read by subject.
- **TMPDIR:**
  - The proof is at run.log lines 1099-1115: the stand-in prints yes, the control prints no, and there is a refusing control.
  - Before and after counts are printed for every run.
  - The one leftover is at lines 2934-2936 (tier c names host: 2 files, 955 bytes, removed), and the summary at lines 3233-3235 shows that as the only one.
- **Zip64 check before tier c counts:** run.log lines 1093-1096 give 5025024826 against 2^32, with the truncated control 730057530.
- **Prediction beside measurement:**
  - Memory class: 48 compared, 0 disagreements.
  - Tier c outcome: 48 runs, 2 disagreements (link on both labels).
- **Throughput:** per run and per label and tier, lines 3975-4144.
- **32-bit control:** lines 4145-4248, with the planted truncation caught.
- **LibreOffice on tier a:** all 48 rows read lo ok and lo_in ok. Tiers b and d show lo not-run in the records.
- **Host xlsx-diff on tier c for both labels:** 24 tier c i686 commands use `<build>/target/release/xlsx-diff`.
- **Counts:** 96 run directories and 96 pairs (lines 4252-4255), 192 runs.
- **Logs end clean:** every log ends all as expected, all four .stderr files are 0 bytes, and the scan reads RESULT: PASS.

Gaps:
1. **The xlsx-diff sha256 is not in the records.** "The runs" says "the record names the binary's sha256 as the runner prints it". No record file contains any 64-hex string (0 of 192 files). The runner writes `xlsx_diff_sha256` only to run.<label>.tsv in the private run directories (passthrough-run.sh, lines 26 and 178), and the record script drops it. run.log line 1047 prints the build-time sha 1445523d..., and line 1052 asserts "the record names it by the sha256 the runner prints". That claim is false for the records. The binary is identifiable only by combining the build hash in run.log with the command paths.
2. **The records show `leftover` as no for tier c names host.** The runner's `leftover` column only looks in the run directory (`$dir/.out.*.tmp`), so it misses TMPDIR. run.log is the only place the leftover is recorded.

**Spot check of records against run.log:** I compared all 192 label rows of the 96 .txt files with run.log's per-tier tables on outcome, peak_rss_kb, in_bytes and out_bytes. All 192 agree. I also read the c-names and c-edit-formula .md and .txt files in full, and b-edit's header. They agree with run.log.

**Q3. Does every number and quoted string in report-draft.md appear in a primary? PARTLY**

Confirmed by recomputation:
- **Samples:** 24 less 5 loads gives 19 streaming samples.
- **Tier b load peaks:** min 173944 (names i686), max 179180 (comment host).
- **Grid-error samples:** 14 (edit, style, font, border, format, link, validate, scale, table, comment, image, cf, date, formula).
- **Tier c outcomes:** errors are 14x2 plus insert-rows x2 = 30. ok is 8 samples x2 = 16. Plus 1 crash and 1 panic.
- **Tier c ok peaks:** maximum 7864 kB (remove host).
- **Rewrites:** five samples rewrite the sheet entry (status changed in the 32-bit control): edit-formula, delete-rows, insert-columns, delete-columns and protect. Their RSS is under 8 MB on i686.
- **Throughput** recomputes to the report's figures:

  | tier | host | i686 |
  | --- | --- | --- |
  | a | 266446040 / 92.13 = 2.89 | / 138.10 = 1.93 |
  | b | 2.90 | 1.95 |
  | d | 3.29 | 2.21 |
  | c | 9324684743 / 1928.60 = 4.83 | / 2905.30 = 3.21 |

- **Sizes:** in_bytes 1159829654; sheet entry out sizes run from 4985617758 to 5026073402.
- **Times:** run start 21:11Z and end 05:50Z.
- **Leftover:** 2 files, 955 bytes.
- **Source and commits:** the link sample's code at tools/emend-copy/src/main.rs lines 3034-3038 (`max_row + 1`, then `set_cell`) is correct at 8d05845. Commit ids, parents and doc hashes are correct.

Not in any primary, or in disagreement:
1. **The names failure messages are in no input.** report-draft.md line 27 quotes "memory allocation of 2198929408 bytes failed" and "capacity overflow". Neither string appears in any file in <K>. run.log prints the message column for both rows as "(not printed: not a constant's shape)", and the records carry no message. These quotes must come out of the report, or a log must print them.
2. **"Peak RSS stays under 8 MB on both tiers" is only true in MiB.** The report's streaming line says this, but props i686 on tier a peaks at 8004 kB. That is under 8 MiB but not under 8 MB in the decimal sense. The throughput lines in the same report use MB = 10^6 (11139024 / 3.70 = 3.0). The report should say "under 8 MiB", or "at most 8004 kB".
3. **"The crate's temporaries" and "Drop does not run on abort" are inference.** run.log only counts the 2 files and 955 bytes; it does not identify them. These should be labelled as the code's reading.
4. **"The survey missed that call" cannot be checked from these inputs.** The P1 record is not among them. run.log does show link predicted ok.
5. **"Each at its first attempt" is consistent with the logs but not proven by them.** RERUN_NOTE is unset in gate.log, test.log and run.log, which supports it.
6. **"Test.log: the new test runs by name ... features off and on" understates what ran.** It also ran with `--all-features` (test.log line 3664). This is a scope note, not an error.

Other things that look wrong or need a planning ruling:
- **The i686 "capacity overflow" panic may fall under the spec's defect list.** "What a finding leads to" lists "a panic" and "a u32 overflow" as defects. The spec allows "a panic on allocation" as a kill, and ruling 7 calls names' failure a hold in the tool. Planning should rule explicitly whether a capacity-overflow panic (a 32-bit size computation, not an allocation failure) counts as that kill. Also, derive.py classes it "killed", not ruling 7's own class "holds in the tool".
- **The spec's "32-bit" sentence is false for tier c, as the report says.** The report's finding 3 is correctly stated: in_bytes and out_bytes are about 1.16e9, under 2^32. The control was applied to the sheet entry sizes instead. Planning needs to rule on that substitution.
- **run.log line 1052's claim about the records** is the false assertion described under Q2, gap 1.