- max 2 golyo
- 5 loves
- .2 esellyel talal

1. Mekkora esellyel el tul?
```
0 talalat: .8^5  
1 talalat: 5*.2*.8^4  
2 talalat: (5 2)*.2^2*.8^3 = 10*.2^2*.8^3  
P(0v1v2 talalat)=.8^5+5*.2*.8^4+10*.2^2*.8^3=0.94208
```
**94.208%** esellyel jut ki elve a partra egy ranger.

2. Hany szazalekot nem talalt golyo?
.8^5*100=**32.768%**-ot nem talalt golyo

3. Atlagosan hany golyo talalt el egy rangert?
```
3 talalat: (5 3)*.2^3*.8^2 = 10*.2^3*.8^2
4 talalat: (5 4)*.2^4*.8 = 5*.2^4*.8
5 talalat: .2^5

0 -> .32768
1 -> .4096
2 -> .2048
3 -> .0512
4 -> .0064
5 -> .00032
sum = 1 :)

avg = (0*.32768+1*.4096+2*.2048+3*.0512+4*.0064+5*.00032) / 1 = 1
```
Egy partraszallot atlagosan **1** golyo talal el.
