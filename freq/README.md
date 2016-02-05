# Freq

Counts the frequencies of words read from the standard input, and print
a sorted frequency table.

Author: James Whang (syw973, sungyoonwhang2017@u.northwestern.edu)

Assumptions:
    - Punctuations ( , | . | ! | ? ) are stripped
        ex) 'hello, world' gives the same result as 'hello world!'
        
    - All words are lowercased
    
    - Quotations will be also stripped
    
    - Words containing quotes will be regarded as a single word.
        ex1) I'm != I m
        ex2) I'm == Im 
        
    - Dashes won't be stripped (-, --, whatever)

