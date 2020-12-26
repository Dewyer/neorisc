### THE PLAN IS SIMPLE

1. Parser --> Lexikai elemzés
   - Létrejönnek hibák
   - Ha nincsenek hibák megyünk tovább a fordításra
2. Fordítás RiscJson-ba
   - Lexikailag helyes source-lineokat fordít le 
   - Szintaktikai ellenörzést végez -> PURE
   - Elvégzi a preprocesszor műveleteket
      - DEF -> timelined symbols are replaced by the values
      - CODE -> Code és adat felcserélhető legyen csak ennyi
      - Adatmemőria felépítése:
         - DATA részben csak DB és label lehet, és abból kell csinálni egy 128*8 bites memóriát
      - Labelek szombólumai címre cserélése