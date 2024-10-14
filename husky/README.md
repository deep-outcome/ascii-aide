# ascii-aide
Command line ASCII table with views into it. See also [husky-lib](https://github.com/bravequickcleverfibreyarn/ascii-aide/tree/main/husky-lib).

```console
> husky --help

        @****************************************************************************************************@
        :     American Standard Code for Information Interchange table aide is faithful as polar dog is.     :
        @~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~@


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
        
        No parameter is same as --help. First known non-optional parameter is considered function match.
        
        EXTRA
        ----------------------------
        -v       | version info
        -r       | references
```

```console
> husky -s

        @****************************************************************************************************@
        :     American Standard Code for Information Interchange table aide is faithful as polar dog is.     :
        @~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~@


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