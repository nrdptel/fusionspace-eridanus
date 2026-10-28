# Pick a motor

Shortlist the motors that fit your rocket, fly it on each one, and choose an ejection delay, all
with `hpr` on the command line.

**This guide shortlists the motors that fit a rocket, flies it on each, and picks an ejection
delay, all with `hpr` on the command line.** It's for a flier choosing a motor for a design they
already have.

> **How far to trust it.** HPR Sim's flights here are estimates from a model. A motor's data, as
> `hpr motors show` says, is copied from its curve file, not measured. Against 53 logged flights of
> fliers' own designs, HPR Sim's apogee averaged 8.7% above the logged one. It was within 10% on
> 26 and read low on 13, by at most 20%.
>
> `hpr sim` gives these figures under each flight below. [Accuracy](accuracy.md) says how close it
> comes elsewhere. The outputs on this page are made by running each command, and CI checks that
> they still match what `hpr` prints. The motor's printed data and your range safety officer have
> the last word.

The steps:

1. [List the motors that fit](#list-the-motors-that-fit).
2. [Fly the rocket on each](#fly-the-rocket-on-each).
3. [Compare them](#compare-them).
4. [Choose the delay](#choose-the-delay).
5. [Put the choice in your design](#put-the-choice-in-your-design).

The examples use the guides' own rocket,
[`level-1.ork`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/guides/level-1.ork),
with a 38 mm motor mount. [Fly your .ork](fly-your-ork.md) introduces it.

## List the motors that fit

**Start from the mount's diameter: a motor's case must match it.** In OpenRocket, the motor mount
tube's inner diameter is the size. A 38 mm mount takes 38 mm motors. `hpr motors list` filters
the 32 motors built into the simulator by diameter, impulse class or maker:

<!-- cli: example `hpr motors list --diameter 38`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors list --diameter 38
211 motors from ThrustCurve.org's solid motor catalog: every solid motor's record and every data file but a restricted one, captured 2026-10-28; size and delays from each motor's default file's header, the rest from its curve, or from ThrustCurve.org's record where no file of it is bundled.

designation   maker     class    dia mm (in)     len mm (in)  impulse N·s   avg N  burn s  delays
G339N         AeroTech  G          38 (1.50)       97  (3.8)        112.1   319.9    0.35  0
G67R          AeroTech  G          38 (1.50)      106  (4.2)        112.8    70.7    1.60  4-6-8-9-10-12-13-15-17
HP-G69-SF     Loki      G          38 (1.50)      127  (5.0)        110.8    70.8    1.57  5-7-9-11-13-15-17
G61W          AeroTech  G          38 (1.50)    106.7  (4.2)        110.8    58.8    1.88  6-10-14
G70-CT        Loki      G          38 (1.50)    122.2  (4.8)        119.0    69.2    1.72  8-18
HP-G94-IB     Loki      G          38 (1.50)    122.2  (4.8)        123.4    94.2    1.31  8-18
117G69-14A    Cesaroni  G          38 (1.50)      126  (5.0)        116.3    61.1    1.90  14-11-9-7-5
G80-LW        Loki      G          38 (1.50)      127  (5.0)        120.3    82.4    1.46  5-8-10-13-17
127G46-11A    Cesaroni  G          38 (1.50)      127  (5.0)        124.1    46.2    2.69  11,8,6,4,2
128G185-12A   Cesaroni  G          38 (1.50)      127  (5.0)        127.1   184.8    0.69  3,5,7,9,12
129G79-13A    Cesaroni  G          38 (1.50)      127  (5.0)        129.7    79.8    1.63  6-8-10-13
G66-LR        Loki      G          38 (1.50)    122.2  (4.8)        135.7    67.1    2.02  5-15
134G60-14A    Cesaroni  G          38 (1.50)      125  (4.9)        139.2    68.1    2.05  0
G69N          AeroTech  G          38 (1.50)      106  (4.2)        136.3    72.1    1.89  1000
137G58-13A    Cesaroni  G          38 (1.50)      127  (5.0)        137.1    57.0    2.40  5-7-8-10-12-13
141G115-13A   Cesaroni  G          38 (1.50)      127  (5.0)        140.8   115.7    1.22  13
141G78-15A    Cesaroni  G          38 (1.50)      126  (5.0)        142.7    78.6    1.82  15-12-10-8-6
150G50-15A    Cesaroni  G          38 (1.50)      127  (5.0)        150.1    50.1    3.00  6-8-11-12-15
H73J          AeroTech  H          38 (1.50)      152  (6.0)        188.3    70.2    2.68  6
H100-SF       Loki      H          38 (1.50)      178  (7.0)        200.9   104.4    1.92  5-8-10-13-17
H283ST-15A    AeroTech  H          38 (1.50)      155  (6.1)        199.1   269.7    0.74  7-11-15
H148R         AeroTech  H          38 (1.50)      152  (6.0)        215.2   144.8    1.49  0
H130W         AeroTech  H          38 (1.50)      152  (6.0)        210.2   135.0    1.56  14-12-10-8-6
H669N         AeroTech  H          38 (1.50)      152  (6.0)        219.7   675.7    0.33  0
H123W         AeroTech  H          38 (1.50)      154  (6.1)        235.2   104.6    2.25  0
H225BL        Gorilla   H          38 (1.50)      248  (9.8)        313.5   249.6    1.26  1000
H100W_DMS     AeroTech  H          38 (1.50)    157.5  (6.2)        228.4    99.7    2.29  14
H242T         AeroTech  H          38 (1.50)      152  (6.0)        243.9   168.7    1.45  10
232H123-14A   Cesaroni  H          38 (1.50)      186  (7.3)        234.6   125.5    1.87  14
H219T         AeroTech  H          38 (1.50)      155  (6.1)        234.6   224.2    1.05  14
H90-LR        Loki      H          38 (1.50)      178  (7.0)        237.1    93.4    2.54  5-8-10-14
H144-LW       Loki      H          38 (1.50)      178  (7.0)        239.2   153.3    1.56  5-8-10-13-17
H125-CT       Loki      H          38 (1.50)    177.8  (7.0)        241.7   125.0    1.93  8-18
247H143-13A   Cesaroni  H          38 (1.50)      186  (7.3)        247.1   143.8    1.72  4-6-8-10-13
H48           Ellis     H          38 (1.50)      200  (7.9)        249.6    57.1    4.37  8-100
255H400-13A   Cesaroni  H          38 (1.50)      186  (7.3)        255.1   400.3    0.64  4,6,8,10,14
H153          Cesaroni  H          38 (1.50)      186  (7.3)        258.7   150.6    1.72  4-6-8-10-13
H160-LB       Loki      H          38 (1.50)      178  (7.0)        255.4   177.8    1.44  7-9-12-15
261H120-14A   Cesaroni  H          38 (1.50)      186  (7.3)        260.6   122.0    2.14  14-11-9-7-5
H112J         AeroTech  H          38 (1.50)      202  (8.0)        260.7    85.5    3.05  6-10
269H110-14A   Cesaroni  H          38 (1.50)      186  (7.3)        270.6   106.5    2.54  5-7-9-11-13-14
273H225-14A   Cesaroni  H          38 (1.50)      186  (7.3)        273.9   229.0    1.20  14
276H152-15A   Cesaroni  H          38 (1.50)      186  (7.3)        279.2   153.0    1.82  15-12-10-8-6
H178DM        AeroTech  H          38 (1.50)      191  (7.5)        285.7   174.5    1.64  14
266H125-12A   Cesaroni  H          38 (1.50)      186  (7.3)        268.5   130.9    2.05  3-5-7-9-12
286H100-15A   Cesaroni  H          38 (1.50)      186  (7.3)        286.4   104.0    2.75  6-8-11-12-15
H45W          AeroTech  H          38 (1.50)      194  (7.6)        288.6    40.8    7.07  0
HP-H550ST     AeroTech  H          38 (1.50)      206  (8.1)        313.4   552.8    0.57  14
H186RT        Gorilla   H          38 (1.50)      248  (9.8)        315.0   189.1    1.67  18
H500-LW       Loki      H          38 (1.50)      292 (11.5)        319.8   517.5    0.62  5-7-9-12-15
I218R         AeroTech  H          38 (1.50)      191  (7.5)        319.1   219.2    1.46  6-10-14
320H565-14A   Cesaroni  H          38 (1.50)      245  (9.6)        319.4   568.7    0.56  5-7-9-11-14
H170M         AeroTech  H          38 (1.50)      191  (7.5)        318.0   165.4    1.92  2,4,6,8,10,14
H999N         AeroTech  H          38 (1.50)      203  (8.0)        320.0   982.7    0.33  0
H550ST        AeroTech  H          38 (1.50)      213  (8.4)        313.4   552.8    0.57  14
HP-H45W       AeroTech  I          38 (1.50)    202.7  (8.0)        321.7    52.4    6.13  10
I235WC        Gorilla   I          38 (1.50)      249  (9.8)        324.0   235.0    1.40  P
I180W         AeroTech  I          38 (1.50)      191  (7.5)        326.3   182.6    1.79  14-12-10-8-6
I223GT        Gorilla   I          38 (1.50)      248  (9.8)        341.0   231.9    1.47  18
I161W         AeroTech  I          38 (1.50)      191  (7.5)        332.7   160.0    2.08  0
I175WS        AeroTech  I          38 (1.50)      214  (8.4)        333.2   177.5    1.88  5-9-13
HP-I140W      AeroTech  I          38 (1.50)    202.7  (8.0)        339.3   137.2    2.47  14-12-10-8-6
338I180-14A   Cesaroni  I          38 (1.50)      243  (9.6)        339.9   179.9    1.89  5-7-9-11-14
I357T         AeroTech  I          38 (1.50)      203  (8.0)        356.2   261.3    1.36  14
I150          Ellis     I          38 (1.50)      229  (9.0)        389.9   163.4    2.39  0
I245G         AeroTech  I          38 (1.50)  192.532  (7.6)        350.5   250.9    1.40  0-6-10-14
GG-38-390     AMW       I          38 (1.50)      258 (10.2)        353.5   244.2    1.45  0
SK-38-390     AMW       I          38 (1.50)      249  (9.8)        351.7   219.8    1.60  20
WW-38-390     AMW       I          38 (1.50)      249  (9.8)        349.8   196.9    1.78  17
364I212-14A   Cesaroni  I          38 (1.50)      245  (9.6)        364.3   214.6    1.70  5-7-9-11-14
I225FJ        AeroTech  I          38 (1.50)      202  (8.0)        350.5   206.7    1.70  6-10-14
396I195-16A   Cesaroni  I          38 (1.50)      245  (9.6)        395.3   196.5    2.01  7,9,11,13,16
370I566-15A   Cesaroni  I          38 (1.50)      245  (9.6)        370.2   570.9    0.65  6,8,10,12,15
I170S         KBA       I          38 (1.50)      258 (10.2)        374.0   181.9    2.06  14
I40N-P        AeroTech  I          38 (1.50)      203  (8.0)        377.4    38.1    9.91  P
I154J         AeroTech  I          38 (1.50)      250  (9.8)        384.0   123.6    3.11  0
I205          Cesaroni  I          38 (1.50)      245  (9.6)        383.5   205.9    1.86  5-7-9-11-14
382I170-14A   Cesaroni  I          38 (1.50)      242  (9.5)        417.3   198.6    2.10  0
I280F         KBA       I          38 (1.50)      258 (10.2)        382.5   263.0    1.45  14
I49N          AeroTech  I          38 (1.50)      184  (7.2)        384.1    49.2    7.81  P
BB-38-390     AMW       I          38 (1.50)      258 (10.2)        389.3   279.2    1.39  0
I300F         Kosdon    I          38 (1.50)      254 (10.0)        390.7   318.8    1.20  P
395I55-9A     Cesaroni  I          38 (1.50)  245.001  (9.6)        395.2    55.4    7.13  9,6,4,2
408I345-15A   Cesaroni  I          38 (1.50)      245  (9.6)        398.7   345.0    1.16  15
I316-SF       Loki      I          38 (1.50)      292 (11.5)        474.0   326.7    1.45  5-8-10-13-17
411I175-14A   Cesaroni  I          38 (1.50)      245  (9.6)        411.4   174.1    2.36  6-8-9-11-12-13
413I236-17A   Cesaroni  I          38 (1.50)      245  (9.6)        414.5   233.2    1.78  17-14-12-10-8
I392BL        Gorilla   I          38 (1.50)      370 (14.6)        416.1   392.0    1.20  P
419I800-15A   Cesaroni  I          38 (1.50)      303 (11.9)        415.5   802.4    0.52  6,8,10,12,16
I285R         AeroTech  I          38 (1.50)      250  (9.8)        415.6   292.6    1.42  0
I1299N        AeroTech  I          38 (1.50)      249  (9.8)        424.5  1291.4    0.33  1000
I195J         AeroTech  I          38 (1.50)      298 (11.7)        472.5   201.3    2.35  10
I300T         AeroTech  I          38 (1.50)      250  (9.8)        436.8   296.7    1.47  6-10-14
I170DH        Kosdon    I          38 (1.50)      314 (12.4)        433.2   167.3    2.59  11
434I223-14A   Cesaroni  I          38 (1.50)      302 (11.9)        435.1   224.5    1.94  5-7-9-11-14
I211W         AeroTech  I          38 (1.50)      240  (9.4)        430.8   204.0    2.11  0
I134          Ellis     I          38 (1.50)      355 (14.0)        445.0    94.7    4.70  15
I305FJ        AeroTech  I          38 (1.50)      298 (11.7)        443.9   293.8    1.51  6-10-14
I160          Ellis     I          38 (1.50)      280 (11.0)        493.7   153.5    3.22  0
I59WN         AeroTech  I          38 (1.50)      232  (9.1)        486.7    59.7    8.15  P
486I287-15A   Cesaroni  I          38 (1.50)      303 (11.9)        486.4   286.6    1.70  6-8-10-12-15
I110-LW       Loki      I          38 (1.50)      292 (11.5)        485.0   128.3    3.78  5-8-12
I210-LR       Loki      I          38 (1.50)      292 (11.5)        493.4   228.8    2.16  5-8-10-14
I405-LW       Loki      I          38 (1.50)      292 (11.5)        499.9   435.7    1.15  5-8-10-13-17
I426LB        Loki      I          38 (1.50)      307 (12.1)        509.1   429.8    1.18  P
512I285-15A   Cesaroni  I          38 (1.50)      303 (11.9)        509.5   273.6    1.86  6-8-10-12-15
517I255-16A   Cesaroni  I          38 (1.50)      303 (11.9)        515.8   255.3    2.02  16,13,11,9,7
I377-CT       Loki      I          38 (1.50)      292 (11.5)        525.8   377.9    1.39  8-18
I324RT        Gorilla   I          38 (1.50)      369 (14.5)        529.1   323.9    1.63  1000
I430-LB       Loki      I          38 (1.50)      292 (11.5)        522.3   436.6    1.20  7-9-12-15
I420F         Kosdon    I          38 (1.50)      314 (12.4)        532.9   437.7    1.20  12
I220S         Kosdon    I          38 (1.50)      314 (12.4)        534.4   224.3    2.40  9
538I303-16A   Cesaroni  I          38 (1.50)      303 (11.9)        539.3   294.8    1.83  16-13-11-9-7
I327DM        AeroTech  I          38 (1.50)      337 (13.3)        542.8   333.2    1.63  14
I366R         AeroTech  I          38 (1.50)      298 (11.7)        539.3   366.5    1.47  0
I230          Ellis     I          38 (1.50)      331 (13.0)        619.0   224.5    2.76  0
540I470-15A   Cesaroni  I          38 (1.50)      303 (11.9)        539.5   484.2    1.11  15
543I297-15A   AMW       I          38 (1.50)      360 (14.2)        540.9   296.8    1.82  6-8-10-12-15
548I242-15A   Cesaroni  I          38 (1.50)      303 (11.9)        550.3   242.5    2.27  6-8-9-11-13
I389GT        Gorilla   I          38 (1.50)      369 (14.5)        555.7   396.7    1.40  18
I301W         KBA       I          38 (1.50)    369.6 (14.6)        590.0   310.7    1.90  18
I240          Cesaroni  I          38 (1.50)      302 (11.9)        549.2   268.1    2.05  0
I462WC        Gorilla   I          38 (1.50)      370 (14.6)        556.7   462.0    1.20  P
HP-I280DM     AeroTech  I          38 (1.50)    356.4 (14.0)        557.4   283.0    1.97  14-12-10-8-6
567I125-10A   Cesaroni  I          38 (1.50)      367 (14.4)        569.5   129.1    4.41  10-7-5-3
I435T         AeroTech  I          38 (1.50)      298 (11.7)        596.1   471.2    1.27  6
I364FJ        AeroTech  I          38 (1.50)      345 (13.6)        551.2   329.5    1.67  6-10-14
SK-38-640     AMW       I          38 (1.50)      369 (14.5)        557.4   314.8    1.77  20
I550R         KBA       I          38 (1.50)    369.6 (14.6)        591.3   560.2    1.06  20
WW-38-640     AMW       I          38 (1.50)      370 (14.6)        583.6   325.2    1.79  17
GG-38-640     AMW       I          38 (1.50)      369 (14.5)        586.8   379.5    1.55  20
I600R         AeroTech  J          38 (1.50)   344.68 (13.6)        640.1   572.6    1.12  6-10-14
I130          Ellis     I          38 (1.50)      330 (13.0)        587.4   139.3    4.22  100
601I350-16A   Cesaroni  I          38 (1.50)      362 (14.3)        600.8   351.0    1.71  7,9,11,13,16
I255DH        Kosdon    I          38 (1.50)      368 (14.5)        602.8   273.4    2.20  10
I284W         AeroTech  I          38 (1.50)      298 (11.7)        590.8   366.8    1.61  10
I132W         AeroTech  I          38 (1.50)      335 (13.2)        594.8   126.2    4.71  0
HP-I500T      AeroTech  I          38 (1.50)      355 (14.0)        622.7   471.7    1.32  14
I360          Cesaroni  I          38 (1.50)      367 (14.4)        626.4   361.9    1.73  15
I450F         KBA       I          38 (1.50)      370 (14.6)        633.7   468.1    1.35  14
I310S         KBA       I          38 (1.50)      368 (14.5)        621.4   286.1    2.17  6-0
I370F         KBA       I          38 (1.50)      368 (14.5)        629.5   362.8    1.73  100
634I540-16A   Cesaroni  I          38 (1.50)      367 (14.4)        634.6   548.3    1.16  7-9-11-13-16
I350R         AeroTech  J          38 (1.50)      356 (14.0)        643.6   408.6    1.58  P
636I216-14A   Cesaroni  I          38 (1.50)      367 (14.4)        636.1   217.6    2.92  5-7-9-11-14
644J94-P      Cesaroni  J          38 (1.50)      367 (14.4)        644.6    95.0    6.79  0
J520F         KBA       J          38 (1.50)      368 (14.5)        645.5   527.2    1.22  6,10,14
648J285-15A   Cesaroni  J          38 (1.50)      367 (14.4)        648.1   288.7    2.25  6,8,10,12,15
J270          Ellis     J          38 (1.50)      384 (15.1)        718.9   260.4    2.76  0
649J335-15A   Cesaroni  J          38 (1.50)      367 (14.4)        646.7   339.3    1.91  15,12,10,8,6
J396-SF       Loki      J          38 (1.50)      406 (16.0)        651.6   413.0    1.58  5-8-10-13-17
650J270-13A   Cesaroni  J          38 (1.50)      367 (14.4)        653.1   269.0    2.43  5-7-8-9-10-11
J340M         AeroTech  J          38 (1.50)      337 (13.3)        652.8   366.1    1.78  2,4,6,8,10,14
BB-38-640     AMW       I          38 (1.50)      369 (14.5)        634.6   435.3    1.46  20
654J316-17A   Cesaroni  J          38 (1.50)      367 (14.4)        653.9   313.2    2.09  17-14-12-10-8
658J357-17A   Cesaroni  J          38 (1.50)      367 (14.4)        657.1   373.8    1.76  17-14-12-10-8
J420R         AeroTech  J          38 (1.50)      337 (13.3)        651.2   424.0    1.54  0
660J381-15A   Cesaroni  J          38 (1.50)      419 (16.5)        659.1   388.1    1.70  15-12-10-8-6
J500ST        AMW       J          38 (1.50)      370 (14.6)        653.9   525.2    1.25  20
J280S         Kosdon    J          38 (1.50)      368 (14.5)        671.1   296.5    2.27  11
J520W         AeroTech  J          38 (1.50)    502.1 (19.8)        686.8   498.9    1.38  5-9-13
J450F         Kosdon    J          38 (1.50)      368 (14.5)        673.3   619.4    1.10  10
J607WC        Gorilla   J          38 (1.50)      370 (14.6)        677.9   607.0    1.15  P
HP-J425R      AeroTech  J          38 (1.50)    356.4 (14.0)        676.8   417.0    1.62  14
J740G         KBA       J          38 (1.50)      370 (14.6)        665.1   818.1    0.81  1000
684J290-15A   Cesaroni  J          38 (1.50)      367 (14.4)        684.2   294.6    2.32  7-8-9-11-12-13
J300          Cesaroni  J          38 (1.50)      360 (14.2)        688.0   333.6    2.06  0
J350W-OLD     AeroTech  J          38 (1.50)      337 (13.3)        689.8   372.9    1.85  0
J350W         AeroTech  J          38 (1.50)      337 (13.3)        669.2   448.2    1.49  P
700J400-16A   Cesaroni  J          38 (1.50)      421 (16.6)        698.9   400.8    1.74  7,9,11,13,16
HP-J270W      AeroTech  J          38 (1.50)    356.4 (14.0)        701.5   274.2    2.56  14-12-10-8-6
J474-CT       Loki      J          38 (1.50)      417 (16.4)        712.5   472.9    1.51  18
J320-LR       Loki      J          38 (1.50)      406 (16.0)        809.7   329.2    2.46  5-8-10-14
J405S         KBA       J          38 (1.50)      476 (18.7)        722.3   387.7    1.86  14
J500G         AeroTech  J          38 (1.50)  335.407 (13.2)        722.7   516.0    1.40  0-6-10-14
J-326-LR      Loki      J          38 (1.50)      417 (16.4)        744.2   336.8    2.21  8-11-15
J605F         KBA       J          38 (1.50)      476 (18.7)        737.9   586.8    1.26  14
J330          Ellis     J          38 (1.50)      433 (17.0)        866.6   328.7    2.64  0
J528-LW       Loki      J          38 (1.50)      406 (16.0)        746.6   652.0    1.15  5-8-10-13-17
J435WS        AeroTech  J          38 (1.50)    365.9 (14.4)        772.1   462.6    1.67  6-10-14
J712-LB       Loki      J          38 (1.50)      406 (16.0)        828.3   711.2    1.16  7-9-12-15
J345S         Kosdon    J          38 (1.50)      476 (18.7)        761.2   361.8    2.10  18
765J330-16A   Cesaroni  J          38 (1.50)      421 (16.6)        765.1   343.7    2.23  7-9-11-13-16
774J410-16A   Cesaroni  J          38 (1.50)      421 (16.6)        777.6   409.6    1.90  16
784J425-16A   Cesaroni  J          38 (1.50)    409.2 (16.1)        785.0   424.3    1.85  16-13-11-9-7
J630F         Kosdon    J          38 (1.50)      476 (18.7)        787.4   659.5    1.17  20
J575FJ        AeroTech  J          38 (1.50)   478.79 (18.9)        796.0   623.1    1.28  6-10-14
J675F         Kosdon    J          38 (1.50)      476 (18.7)        814.5   710.5    1.17  20
819J354-16A   Cesaroni  J          38 (1.50)      421 (16.6)        819.4   353.8    2.32  7-8-9-11-12-13
J360          Cesaroni  J          38 (1.50)      419 (16.5)        815.8   415.7    1.96  0
848J520-16A   Cesaroni  J          38 (1.50)      500 (19.7)        847.7   519.8    1.63  16-13-11-9-7
896J580-17A   Cesaroni  J          38 (1.50)      510 (20.1)        894.8   577.1    1.55  17-14-12-10-8
J850F         Kosdon    J          38 (1.50)      576 (22.7)        899.9   883.8    1.00  21
J650-SF       Loki      J          38 (1.50)      625 (24.6)        947.9   672.1    1.41  7-9-12-15
949J150-P     Cesaroni  J          38 (1.50)  499.999 (19.7)        943.3   148.9    6.34  0
970J394-13A   Cesaroni  J          38 (1.50)      500 (19.7)        972.2   393.7    2.47  4-7-9-11-13-14
J570W         AeroTech  J          38 (1.50)      479 (18.9)       1033.8   535.7    1.93  0
J825R         AeroTech  J          38 (1.50)      479 (18.9)        962.2   901.6    1.07  14
985J595-16A   Cesaroni  J          38 (1.50)      500 (19.7)        986.3   595.5    1.66  16-13-11-9-7
J530S         Kosdon    J          38 (1.50)      737 (29.0)        991.7   549.4    1.80  10
999J600-16A   Cesaroni  J          38 (1.50)      510 (20.1)        997.9   603.4    1.65  16-13-11-9-7
1008J420-15A  Cesaroni  J          38 (1.50)      500 (19.7)       1008.2   436.4    2.31  6-8-10-12-15
1013J453-16A  Cesaroni  J          38 (1.50)      500 (19.7)       1018.0   453.3    2.25  16-13-11-9-7
J975F         Kosdon    J          38 (1.50)      740 (29.1)       1033.4  1018.5    1.03  23,P
J228          Ellis     J          38 (1.50)      562 (22.1)       1114.6   222.8    5.00  6
1115J530-15A  Cesaroni  J          38 (1.50)      500 (19.7)       1115.2   547.6    2.04  6-8-10-12-15
J510W         AeroTech  J          38 (1.50)      584 (23.0)       1180.5   548.2    2.15  P
J1000-LW      Loki      J      38.001 (1.50)      625 (24.6)       1220.4  1061.4    1.15  7,9,12,15
J1026CT       Loki      J          38 (1.50)    625.5 (24.6)       1263.8  1040.7    1.21  P
K1127-LB      Loki      K          38 (1.50)    625.5 (24.6)       1285.8  1040.1    1.24  P
K627LR        Loki      K          38 (1.50)    625.5 (24.6)       1526.4   624.9    2.44  5-7-11-14

17 of these have no curve file bundled, so their figures are ThrustCurve.org's record's and `hpr sim --motor` can't fly them: I235WC, I300F, I392BL, I170DH, I420F, I220S, I462WC, I255DH, J280S, J450F, J607WC, J345S, J630F, J675F, J850F, J530S and J975F.
```

<!-- cli: end -->

The figures are worked out from each motor's public-domain ThrustCurve.org curve file, the curve
the simulator flies ([The bundled motors](physics/motor.md#the-bundled-motors)). The `delays` column
lists the ejection delays as the file's header writes them, in seconds after burnout. A `-` separates
the settings: `5-9-13` is 5, 9 and 13 s, and `8-18` is 8 and 18 s. The header can list fewer delays
than the maker sells, so check the motor's instructions. `P` (or `1000`) is a plugged motor with no
ejection charge.

A Level 1 certification flight uses an H or I motor (NAR and Tripoli's Level 1 rules;
[Check stability for a certification flight](stability-for-certification.md#what-the-rules-ask)
quotes them).

**Many more motors exist than the 32 built in.** Name any motor ThrustCurve.org lists, such as
`--motor H128W`. The simulator fetches its thrust curve once online and keeps it
([Fetch a motor the simulator doesn't have](fly-your-ork.md#fetch-a-motor-the-simulator-doesnt-have)).
`hpr motors search` lists what vendors have in stock and at what price
([Motors you can buy](cli.md#motors-you-can-buy)). `hpr motors show` gives one motor's figures as
the simulator reads its curve:

<!-- cli: example `hpr motors show I175WS`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr motors show I175WS
I175WS (AeroTech), from the bundled catalog
  impulse class    I (320.01–640 N·s)
  total impulse    333.2 N·s
  average thrust   177.5 N measured; 175 N nominal, as the designation names it
  peak thrust      252.6 N
  burn time        1.88 s, from 0.023 s to 1.900 s (NFPA 1125, 5% of peak)
  propellant       WS, the maker's code for Super White Lightning, as ThrustCurve.org names it
  propellant mass  168.0 g (5.9 oz) of 348.0 g (12.3 oz) loaded
  casing           38 mm (1.50 in) across, 214 mm (8.4 in) long
  delays           5 s, 9 s, 13 s
  curve file       https://www.thrustcurve.org/simfiles/5f4294d20002e900000008bf/ (flown)
                   RASP (.eng), from a certification test, public domain
                   passes the 1% check: total impulse +0.05%, average thrust +0.57%, burn time -0.66%
  curve file       https://www.thrustcurve.org/simfiles/5f9245ad1bca580004171795/
                   RockSim (.rse), from a user, public domain
                   passes the 1% check: total impulse +0.05%, average thrust +0.57%, burn time -0.66%

How far to trust it. Copied from ThrustCurve.org's curve file, downloaded 2026-10-28, not measured
by HPR Sim: the size, masses and delays from the file's header, the impulse, thrusts and burn time
from its curve, which can differ from the maker's rated figures. Each file is checked against
ThrustCurve.org's figures and OpenRocket's reading of its impulse and peak thrust, not against the
maker's data: checks of the file, not of the motor. The motor's printed data and its maker's
instructions come first, and the RSO decides.
More: https://hpr.fusionspace.co/pick-a-motor.html
```

<!-- cli: end -->

## Fly the rocket on each

**`--motor` flies the design with another motor in its mount, and `--delay` sets that motor's
ejection delay.** Without `--delay`, the motor fires no charge. A parachute set to open on the
charge, as the guide rocket's is, then never opens. Give one of the motor's delays, in seconds, or
`P` for a plugged motor when your electronics fire the charges. This flies the guide rocket on a
Loki H125, with a 10 s delay:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork --motor H125-CT --delay 10`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork --motor H125-CT --delay 10
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10] with --motor H125-CT --delay 10
help: --config flies the others, by number or name: 2 [I175WS-9]
a simulated flight, not a measurement

static margin         1.59 calibers off the rail; least 1.59 calibers, at 0.17 s, before apogee
CG and CP             0.818 m (32.2 in) and 0.923 m (36.3 in) aft of the nose tip, off the rail
apogee                914.5 m (3000 ft) above the site at 11.42 s
rail exit speed       25.9 m/s (85 ft/s)
delay                 apogee 9.42 s after burnout; the motor's set delay: 10 s; its charge fires 0.58 s after apogee
descent               4.7 m/s (15 ft/s) at landing under `Parachute`

motor: 1 × H125-CT (from the bundled catalog, as of 2026-10-28) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 12.00 s
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
note: the file's 1 recovery device flies as OpenRocket flies it. Each opens fully at its event, with the file's drag coefficient or OpenRocket's own. Once one opens, the rocket descends as a point under the open devices' drag alone
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower. HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3. If this surface is that smooth, the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found. This flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                 height                    speed
liftoff               0.02 s      0.4 m      (1 ft)     0.0 m/s     (0 ft/s)
rail exit             0.17 s      1.9 m      (6 ft)    25.9 m/s    (85 ft/s)
burnout               2.00 s    303.8 m    (997 ft)   201.2 m/s   (660 ft/s)
apogee               11.42 s    914.5 m   (3000 ft)     0.1 m/s     (0 ft/s)
charge               12.00 s    912.8 m   (2995 ft)     5.6 m/s    (18 ft/s)
deployment           12.00 s    912.8 m   (2995 ft)     5.6 m/s    (18 ft/s)
ground hit          202.71 s      0.0 m      (0 ft)     4.7 m/s    (15 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             217.0 m/s (712 ft/s) at 1.56 s
top Mach number       0.639
landing               0.7 m (2 ft) from the pad at 202.71 s, at 4.7 m/s (15 ft/s)

How far to trust it. Simulated from the design file, not measured. Against 53 logged flights of
fliers' own designs, the simulated apogee averaged 8.7% above the logged one. Each logged apogee is
the height climbed in the day's air. The simulated apogee was within 10% on 26 and read low on 13,
by at most 20%. Plan a waiver or a field's ceiling with room above this apogee; the altimeter's
reading is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

And on the hardest-hitting motor in the list, a Loki I377, with the same delay:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork --motor I377-CT --delay 10`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork --motor I377-CT --delay 10
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10] with --motor I377-CT --delay 10
help: --config flies the others, by number or name: 2 [I175WS-9]
a simulated flight, not a measurement

static margin         1.18 calibers off the rail; least 1.18 calibers, at 0.08 s, before apogee
CG and CP             0.845 m (33.3 in) and 0.923 m (36.3 in) aft of the nose tip, off the rail
apogee                1320.4 m (4332 ft) above the site at 12.02 s
rail exit speed       42.3 m/s (139 ft/s)
delay                 apogee 10.59 s after burnout; the motor's set delay: 10 s; its charge fires 0.59 s before apogee
descent               5.0 m/s (16 ft/s) at landing under `Parachute`

motor: 1 × I377-CT (from the bundled catalog, as of 2026-10-28) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 11.43 s
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
note: the file's 1 recovery device flies as OpenRocket flies it. Each opens fully at its event, with the file's drag coefficient or OpenRocket's own. Once one opens, the rocket descends as a point under the open devices' drag alone
warning: design checks: the motor in `Motor mount tube` reaches 22.0 mm past the mount's forward end (configuration [H170M-10] with --motor I377-CT --delay 10)
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5). So the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are. This flight reaches Mach 1.10 at 1.1 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9). So the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are. This flight reaches Mach 1.10 at 1.1 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag: issue #222: supersonic pressure drag on an ogive nose or airfoil fins is about twice OpenRocket's, and which is right is unresolved. If HPR Sim's drag is high, the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are. This flight reaches Mach 1.10 at 1.1 s (https://github.com/nrdptel/fusionspace-eridanus/issues/222)
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower. HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3. If this surface is that smooth, the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found. This flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                 height                    speed
liftoff               0.00 s      0.3 m      (1 ft)     0.0 m/s     (0 ft/s)
rail exit             0.08 s      1.8 m      (6 ft)    42.3 m/s   (139 ft/s)
burnout               1.43 s    384.0 m   (1260 ft)   335.1 m/s  (1099 ft/s)
charge               11.43 s   1318.2 m   (4325 ft)    10.1 m/s    (33 ft/s)
deployment           11.43 s   1318.2 m   (4325 ft)    10.1 m/s    (33 ft/s)
apogee               12.02 s   1320.4 m   (4332 ft)     0.0 m/s     (0 ft/s)
ground hit          269.86 s      0.0 m      (0 ft)     5.0 m/s    (16 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             371.7 m/s (1219 ft/s) at 1.08 s
top Mach number       1.095
landing               0.8 m (3 ft) from the pad at 269.86 s, at 5.0 m/s (16 ft/s)

How far to trust it. Simulated from the design file, not measured. Against 53 logged flights of
fliers' own designs, the simulated apogee averaged 8.7% above the logged one. Each logged apogee is
the height climbed in the day's air. The simulated apogee was within 10% on 26 and read low on 13,
by at most 20%. Plan a waiver or a field's ceiling with room above this apogee; the altimeter's
reading is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

The `warning` says the I377's case reaches 22.0 mm (0.87 in) past the mount tube's forward end.
The 292 mm (11.5 in) case sits 10 mm (0.39 in) out of the back of the rocket's 260 mm (10 in)
mount tube. The simulator flies it as drawn. Whether it fits the real rocket is for you to check.

`--motor` replaces the motor of the configuration flown. `--config` picks which of the file's
configurations it goes in, and `--mount` picks which mount, if the design has several.

## Compare them

**Read the same lines for each motor: margin, apogee, rail exit speed, delay and descent.**
Between the two flights above:

- The I377 climbs higher. If your field has a ceiling, its waiver sets it, not the simulator.
- The I377 leaves the rail much faster. The safety codes ask for a speed that ensures a stable
  flight but give no number. Competitions set their own, such as 25 m/s (82 ft/s) for the
  Spaceport America Cup
  ([Rail exit speed](stability-for-certification.md#rail-exit-speed) quotes them).
- The stability margin changes with the motor. A motor's mass sits behind the center of gravity.
  So a heavier motor moves the center of gravity aft, toward the center of pressure, and shrinks
  the margin. Check it with every motor you might fly
  ([Check stability for a certification flight](stability-for-certification.md)).
- The I377 takes the rocket past the speed of sound, Mach 1. The H125 stays well below it, and
  the H170M, at Mach 0.842, barely enters the range from Mach 0.8 to 1.2. In that range, HPR Sim's
  center of pressure is up to 2.36 calibers behind NASA's wind-tunnel data
  ([Known gaps](accuracy.md#known-gaps)). So its margin there reads high, and published guidance
  asks for a larger margin
  ([Compare with published guidance](stability-for-certification.md#compare-with-published-guidance)).

To fly a few more motors, change `--motor` and `--delay` and run it again. `--json` prints each
flight as data, for a script that tabulates many.

## Choose the delay

**The `delay` line's first figure is the coast, from burnout to apogee. A delay near it fires the
charge near apogee, where the rocket moves slowest.** The second figure is the delay you gave, and
the third says how far from apogee the charge then fires. In the events table, the `charge` row
gives the speed at that moment. The further the charge is from apogee, the faster the rocket moves
when the parachute comes out.

In the H125 flight above, the coast is 9.42 s, and the 10 s delay fires the charge 0.58 s after
apogee. The H125 comes in delays of 8, 10, 12, 14 and 18 s, more than its file's header lists, so
10 s is the nearest. The simulator flies any delay you give. Give one the maker sells, or one your
motor's delay can be adjusted to.

A delay is a burning fuse, and real ones vary from motor to motor. The simulator flies the stated
time exactly. [Monte Carlo dispersion](monte-carlo.md), in the library, can scatter it.

The guide rocket's own second configuration, `[I175WS-9]`, is an example where the set delay
fires before apogee:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork --config 2`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork --config 2
Level 1 guide rocket (level-1.ork)
configuration 2 of 2: [I175WS-9]
help: --config flies the others, by number or name: 1 [H170M-10]
a simulated flight, not a measurement

static margin         1.67 calibers off the rail; least 1.67 calibers, at 0.14 s, before apogee
CG and CP             0.813 m (32.0 in) and 0.923 m (36.3 in) aft of the nose tip, off the rail
apogee                1100.7 m (3611 ft) above the site at 11.48 s
rail exit speed       28.9 m/s (95 ft/s)
delay                 apogee 9.51 s after burnout; the motor's set delay: 9 s; its charge fires 0.51 s before apogee
descent               4.5 m/s (15 ft/s) at landing under `Parachute`

motor: 1 × I175WS (from the bundled catalog, as of 2026-10-28) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 10.97 s
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
note: the file's 1 recovery device flies as OpenRocket flies it. Each opens fully at its event, with the file's drag coefficient or OpenRocket's own. Once one opens, the rocket descends as a point under the open devices' drag alone
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5). So the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are. This flight reaches Mach 0.83 at 1.8 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9). So the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are. This flight reaches Mach 0.83 at 1.8 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower. HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3. If this surface is that smooth, the apogee, the top speed and the drift read low. The flutter margin and the largest dynamic pressure then look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found. This flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time                 height                    speed
liftoff               0.02 s      0.4 m      (1 ft)     0.0 m/s     (0 ft/s)
rail exit             0.14 s      1.9 m      (6 ft)    28.9 m/s    (95 ft/s)
burnout               1.97 s    385.2 m   (1264 ft)   260.5 m/s   (855 ft/s)
charge               10.97 s   1099.1 m   (3606 ft)     8.0 m/s    (26 ft/s)
deployment           10.97 s   1099.1 m   (3606 ft)     8.0 m/s    (26 ft/s)
apogee               11.48 s   1100.7 m   (3611 ft)     0.0 m/s     (0 ft/s)
ground hit          247.48 s      0.0 m      (0 ft)     4.5 m/s    (15 ft/s)
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             281.3 m/s (923 ft/s) at 1.75 s
top Mach number       0.830
landing               0.7 m (2 ft) from the pad at 247.48 s, at 4.5 m/s (15 ft/s)

How far to trust it. Simulated from the design file, not measured. Against 53 logged flights of
fliers' own designs, the simulated apogee averaged 8.7% above the logged one. Each logged apogee is
the height climbed in the day's air. The simulated apogee was within 10% on 26 and read low on 13,
by at most 20%. Plan a waiver or a field's ceiling with room above this apogee; the altimeter's
reading is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

## Put the choice in your design

**The simulator doesn't change your `.ork` file: set the motor and its delay in OpenRocket, save,
and fly the file again.** Without `--motor`, `hpr sim` flies the motors and delays the file holds,
as [Fly your .ork](fly-your-ork.md) shows. `hpr sim`'s `delay` line checks the delay you set.

## What it leaves out

- Each motor flies one thrust curve, ThrustCurve.org's, and its stated delay exactly. Real motors
  differ a little from their published curves.
- The simulator's design checks flag a motor wider than its mount, and warn on one longer than its
  tube. They don't know your motor retainer, thrust ring or closure.
- The simulator gives no verdict: it ranks nothing as safe or unsafe. The motor's printed data, the
  safety codes and your range safety officer decide.
