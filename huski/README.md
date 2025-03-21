# huski
Command line ASCII table with views into it. See also [huski-lib](https://github.com/deep-outcome/ascii-aide/tree/main/huski-lib).

```console
> huski --help

       @***************************************************************************************************@
        :    American Standard Code for Information Interchange table aide is faithful as arctic dog is.    :
        @~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~@


        SUBSETS
        ----------------------------
        -p       | printable
        -c       | control
        -lc      | capital letters
        -ls      | small letters
        -l       | all letters
        -d       | digits
        -s       | symbols
        
        SET
        ----------------------------
        -t       | whole table

        GENERAL
        ----------------------------
        --help   | this help
        -nt:base | number type, defaults to nt:10 = decimal, supports: binary, octal, decimal, hexadecimal
        -tt:type | table type, defaults to tt:c = classic order, supports: s — special, c — classic 
                 | if -tt:s prints subset ordered-table in order: lc,ls,d,s,c, works only with -t
        
        No parameter is same as --help. First known non-optional parameter is considered function match. 
        Similarly, first valid optional parameter is considered match.
        
        EXTRA
        ----------------------------
        -v       | version info
        -r       | references
```

```console
> huski -s

        @***************************************************************************************************@
        :    American Standard Code for Information Interchange table aide is faithful as arctic dog is.    :
        @~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~@


          NUMERIC  | HUMAN | DESCRIPTION
        -------------------------------------------------
            32     |       | Space
            33     |   !   | Exlamation mark
            34     |   "   | Double quotation mark
            35     |   #   | Number sign
            36     |   $   | Dollar sign
            37     |   %   | Percent sign
            38     |   &   | Ampersand
            39     |   '   | Apostrophe
            40     |   (   | Left parenthesis
            41     |   )   | Right parenthesis
            42     |   *   | Asterisk
            43     |   +   | Plus sign
            44     |   ,   | Comma
            45     |   -   | Hyphen/Minus sign
            46     |   .   | Period
            47     |   /   | Solidus
            58     |   :   | Colon
            59     |   ;   | Semicolon
            60     |   <   | Less-than sign
            61     |   =   | Equals sign
            62     |   >   | Greater-than sign
            63     |   ?   | Question mark
            64     |   @   | At sign
            91     |   [   | Left bracket
            92     |   \   | Reverse solidus
            93     |   ]   | Right bracket
            94     |   ^   | Circumflex accent
            95     |   _   | Underscore
            96     |   `   | Grave accent
            123    |   {   | Left brace
            124    |   |   | Verical line
            125    |   }   | Right brace
            126    |   ~   | Tilde
```

Special ordering for increased usability and readability.

```console
> huski -t -tt:s

        @***************************************************************************************************@
        :    American Standard Code for Information Interchange table aide is faithful as arctic dog is.    :
        @~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~@


        |  NUMERIC  | HUMAN |  NUMERIC  | HUMAN |  NUMERIC  | HUMAN |  NUMERIC  | HUMAN |  NUMERIC  | HUMAN |
        -----------------------------------------------------------------------------------------------------
        | 65        |A      | 97        |a      | 48        |0      | 32        |       | 0         |NUL    
        | 66        |B      | 98        |b      | 49        |1      | 33        |!      | 1         |SOH    
        | 67        |C      | 99        |c      | 50        |2      | 34        |"      | 2         |STX    
        | 68        |D      | 100       |d      | 51        |3      | 35        |#      | 3         |ETX    
        | 69        |E      | 101       |e      | 52        |4      | 36        |$      | 4         |EOT    
        | 70        |F      | 102       |f      | 53        |5      | 37        |%      | 5         |ENQ    
        | 71        |G      | 103       |g      | 54        |6      | 38        |&      | 6         |ACK    
        | 72        |H      | 104       |h      | 55        |7      | 39        |'      | 7         |BEL    
        | 73        |I      | 105       |i      | 56        |8      | 40        |(      | 8         |BS     
        | 74        |J      | 106       |j      | 57        |9      | 41        |)      | 9         |HT     
        | 75        |K      | 107       |k      | ---       | -     | 42        |*      | 10        |LF     
        | 76        |L      | 108       |l      | ---       | -     | 43        |+      | 11        |VT     
        | 77        |M      | 109       |m      | ---       | -     | 44        |,      | 12        |FF     
        | 78        |N      | 110       |n      | ---       | -     | 45        |-      | 13        |CR     
        | 79        |O      | 111       |o      | ---       | -     | 46        |.      | 14        |SO     
        | 80        |P      | 112       |p      | ---       | -     | 47        |/      | 15        |SI     
        | 81        |Q      | 113       |q      | ---       | -     | 58        |:      | 16        |DLE    
        | 82        |R      | 114       |r      | ---       | -     | 59        |;      | 17        |DC1    
        | 83        |S      | 115       |s      | ---       | -     | 60        |<      | 18        |DC2    
        | 84        |T      | 116       |t      | ---       | -     | 61        |=      | 19        |DC3    
        | 85        |U      | 117       |u      | ---       | -     | 62        |>      | 20        |DC4    
        | 86        |V      | 118       |v      | ---       | -     | 63        |?      | 21        |NAK    
        | 87        |W      | 119       |w      | ---       | -     | 64        |@      | 22        |SYN    
        | 88        |X      | 120       |x      | ---       | -     | 91        |[      | 23        |ETB    
        | 89        |Y      | 121       |y      | ---       | -     | 92        |\      | 24        |CAN    
        | 90        |Z      | 122       |z      | ---       | -     | 93        |]      | 25        |EM     
        | ---       | -     | ---       | -     | ---       | -     | 94        |^      | 26        |SUB    
        | ---       | -     | ---       | -     | ---       | -     | 95        |_      | 27        |ESC    
        | ---       | -     | ---       | -     | ---       | -     | 96        |`      | 28        |FS     
        | ---       | -     | ---       | -     | ---       | -     | 123       |{      | 29        |GS     
        | ---       | -     | ---       | -     | ---       | -     | 124       ||      | 30        |RS     
        | ---       | -     | ---       | -     | ---       | -     | 125       |}      | 31        |US     
        | ---       | -     | ---       | -     | ---       | -     | 126       |~      | 127       |DEL   
```
