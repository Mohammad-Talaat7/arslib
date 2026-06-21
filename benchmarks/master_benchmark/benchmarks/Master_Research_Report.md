# ARS Research Atlas v3.2: Universal Empirical Analysis

## Environment Details
- **CPU:** 8 Cores
- **RAM:** 15864 MB
- **Seed:** 42 | **Reps:** 5

## Benchmarking Results

### Scale: N = 10000

| Type | Distribution | Algorithm | Avg Time | Throughput |
| :--- | :--- | :--- | :--- | :--- |
| i64 | SmallInts | Std Unstable | 72.251µs | 138406388/s |
| i64 | SmallInts | Std Stable | 113.785µs | 87885046/s |
| i64 | SmallInts | ARS Gen 3 | 156.033µs | 64089006/s |
| i64 | SmallInts | ARS Apex | 421.047µs | 23750317/s |
| i64 | BigInts | Std Unstable | 149.646µs | 66824372/s |
| i64 | BigInts | Std Stable | 196.875µs | 50793650/s |
| i64 | BigInts | ARS Gen 3 | 190.858µs | 52394974/s |
| i64 | BigInts | ARS Apex | 288.011µs | 34720896/s |
| i64 | PositiveOnly | Std Unstable | 179.526µs | 55702238/s |
| i64 | PositiveOnly | Std Stable | 195.372µs | 51184407/s |
| i64 | PositiveOnly | ARS Gen 3 | 191.201µs | 52300981/s |
| i64 | PositiveOnly | ARS Apex | 240.699µs | 41545664/s |
| f64 | BigFractions | Std Unstable | 214.918µs | 46529373/s |
| f64 | BigFractions | Std Stable | 346.165µs | 28887958/s |
| f64 | BigFractions | ARS Gen 3 | 253.02µs | 39522567/s |
| f64 | BigFractions | ARS Apex | 385.908µs | 25912911/s |
| f64 | SmallFractions | Std Unstable | 287.955µs | 34727648/s |
| f64 | SmallFractions | Std Stable | 317.06µs | 31539771/s |
| f64 | SmallFractions | ARS Gen 3 | 250.406µs | 39935145/s |
| f64 | SmallFractions | ARS Apex | 319.184µs | 31329891/s |
| f64 | SpecialValues | Std Unstable | 112.299µs | 89047987/s |
| f64 | SpecialValues | Std Stable | 177.951µs | 56195244/s |
| f64 | SpecialValues | ARS Gen 3 | 163.497µs | 61163201/s |
| f64 | SpecialValues | ARS Apex | 325.007µs | 30768568/s |
| String | Chars | Std Unstable | 30.588µs | 32692559/s |
| String | Chars | Std Stable | 35.061µs | 28521719/s |
| String | Chars | ARS Apex | 62.441µs | 16015118/s |
| String | Repeated | Std Unstable | 3.858µs | 259201658/s |
| String | Repeated | Std Stable | 3.477µs | 287604256/s |
| String | Repeated | ARS Apex | 7.345µs | 136147038/s |
| String | Paragraphs | Std Unstable | 86.508µs | 11559624/s |
| String | Paragraphs | Std Stable | 69.154µs | 14460479/s |
| String | Paragraphs | ARS Apex | 121.254µs | 8247150/s |
| Record | HeavyRecords | Std Unstable | 34.223µs | 29220115/s |
| Record | HeavyRecords | Std Stable | 38.371µs | 26061348/s |
| Record | HeavyRecords | ARS Apex | 71.155µs | 14053826/s |


### Scale: N = 100000

| Type | Distribution | Algorithm | Avg Time | Throughput |
| :--- | :--- | :--- | :--- | :--- |
| i64 | SmallInts | Std Unstable | 716.554µs | 139556823/s |
| i64 | SmallInts | Std Stable | 1.035394ms | 96581591/s |
| i64 | SmallInts | ARS Gen 3 | 1.241145ms | 80570763/s |
| i64 | SmallInts | ARS Apex | 2.404434ms | 41589829/s |
| i64 | BigInts | Std Unstable | 1.58205ms | 63209127/s |
| i64 | BigInts | Std Stable | 2.349732ms | 42558044/s |
| i64 | BigInts | ARS Gen 3 | 2.284375ms | 43775649/s |
| i64 | BigInts | ARS Apex | 2.589072ms | 38623877/s |
| i64 | PositiveOnly | Std Unstable | 1.790944ms | 55836475/s |
| i64 | PositiveOnly | Std Stable | 2.408663ms | 41516808/s |
| i64 | PositiveOnly | ARS Gen 3 | 2.276639ms | 43924399/s |
| i64 | PositiveOnly | ARS Apex | 2.5264ms | 39582013/s |
| f64 | BigFractions | Std Unstable | 2.560562ms | 39053926/s |
| f64 | BigFractions | Std Stable | 3.634506ms | 27514055/s |
| f64 | BigFractions | ARS Gen 3 | 2.836956ms | 35249048/s |
| f64 | BigFractions | ARS Apex | 2.504471ms | 39928591/s |
| f64 | SmallFractions | Std Unstable | 2.306684ms | 43352275/s |
| f64 | SmallFractions | Std Stable | 3.257946ms | 30694185/s |
| f64 | SmallFractions | ARS Gen 3 | 2.510035ms | 39840081/s |
| f64 | SmallFractions | ARS Apex | 2.674279ms | 37393256/s |
| f64 | SpecialValues | Std Unstable | 1.348728ms | 74143934/s |
| f64 | SpecialValues | Std Stable | 2.168862ms | 46107128/s |
| f64 | SpecialValues | ARS Gen 3 | 2.409908ms | 41495359/s |
| f64 | SpecialValues | ARS Apex | 2.816454ms | 35505639/s |
| String | Chars | Std Unstable | 252.621µs | 39584990/s |
| String | Chars | Std Stable | 384.208µs | 26027568/s |
| String | Chars | ARS Apex | 397.315µs | 25168946/s |
| String | Repeated | Std Unstable | 32.819µs | 304701544/s |
| String | Repeated | Std Stable | 29.573µs | 338146282/s |
| String | Repeated | ARS Apex | 74.398µs | 134412215/s |
| String | Paragraphs | Std Unstable | 735.746µs | 13591647/s |
| String | Paragraphs | Std Stable | 827.833µs | 12079731/s |
| String | Paragraphs | ARS Apex | 689.941µs | 14493992/s |
| Record | HeavyRecords | Std Unstable | 415.224µs | 24083386/s |
| Record | HeavyRecords | Std Stable | 485.811µs | 20584136/s |
| Record | HeavyRecords | ARS Apex | 377.231µs | 26508956/s |


### Scale: N = 1000000

| Type | Distribution | Algorithm | Avg Time | Throughput |
| :--- | :--- | :--- | :--- | :--- |
| i64 | SmallInts | Std Unstable | 10.087912ms | 99128541/s |
| i64 | SmallInts | Std Stable | 15.361228ms | 65098962/s |
| i64 | SmallInts | ARS Gen 3 | 11.663955ms | 85734212/s |
| i64 | SmallInts | ARS Apex | 25.806655ms | 38749694/s |
| i64 | BigInts | Std Unstable | 21.830328ms | 45807832/s |
| i64 | BigInts | Std Stable | 36.89535ms | 27103686/s |
| i64 | BigInts | ARS Gen 3 | 24.724849ms | 40445140/s |
| i64 | BigInts | ARS Apex | 37.381243ms | 26751384/s |
| i64 | PositiveOnly | Std Unstable | 28.3945ms | 35218088/s |
| i64 | PositiveOnly | Std Stable | 35.937813ms | 27825844/s |
| i64 | PositiveOnly | ARS Gen 3 | 24.419452ms | 40950959/s |
| i64 | PositiveOnly | ARS Apex | 37.080893ms | 26968066/s |
| f64 | BigFractions | Std Unstable | 35.78682ms | 27943248/s |
| f64 | BigFractions | Std Stable | 53.448203ms | 18709702/s |
| f64 | BigFractions | ARS Gen 3 | 34.601416ms | 28900551/s |
| f64 | BigFractions | ARS Apex | 31.405805ms | 31841247/s |
| f64 | SmallFractions | Std Unstable | 36.140335ms | 27669915/s |
| f64 | SmallFractions | Std Stable | 52.757199ms | 18954759/s |
| f64 | SmallFractions | ARS Gen 3 | 36.545079ms | 27363465/s |
| f64 | SmallFractions | ARS Apex | 31.942395ms | 31306356/s |
| f64 | SpecialValues | Std Unstable | 22.478312ms | 44487326/s |
| f64 | SpecialValues | Std Stable | 33.169983ms | 30147739/s |
| f64 | SpecialValues | ARS Gen 3 | 28.418911ms | 35187836/s |
| f64 | SpecialValues | ARS Apex | 29.199468ms | 34247199/s |
| String | Chars | Std Unstable | 4.308869ms | 23207946/s |
| String | Chars | Std Stable | 7.047966ms | 14188490/s |
| String | Chars | ARS Apex | 6.128015ms | 16318497/s |
| String | Repeated | Std Unstable | 521.67µs | 191692065/s |
| String | Repeated | Std Stable | 573.354µs | 174412317/s |
| String | Repeated | ARS Apex | 1.359229ms | 73571120/s |
| String | Paragraphs | Std Unstable | 26.796039ms | 3731894/s |
| String | Paragraphs | Std Stable | 33.160437ms | 3015641/s |
| String | Paragraphs | ARS Apex | 15.007692ms | 6663249/s |
| Record | HeavyRecords | Std Unstable | 6.437791ms | 15533278/s |
| Record | HeavyRecords | Std Stable | 10.306034ms | 9703053/s |
| Record | HeavyRecords | ARS Apex | 5.302617ms | 18858612/s |


## Theoretical Complexity and Stability Summary

| Algorithm | Best Case | Avg Case | Worst Case | Space | Stable |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Std Unstable | O(N) | O(N log N) | O(N log N) | O(log N) | No |
| Std Stable | O(N) | O(N log N) | O(N log N) | O(N) | Yes |
| ARS Gen 1 | O(N log N) | O(N log N) | O(N log N) | O(N) | No |
| ARS Gen 3 | O(N) | O(N) | O(N^2) | O(N) | No |
| ARS Apex | O(N) | O(N) | O(N log N) | O(1) | No |
