<start> ::= '0' | <leading_digit> <digit>*
<leading_digit> ::= '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' 

<parse_start> ::= '+'? <digit>+

where int(str(<start>)) % 2 == 0
where int(str(<parse_start>)) % 2 == 0
