(load "scheme/fileread.scm")  ; file reading helpers

; load in file as a list of lists of words
(define input (read-file-split-spaces "inputs/day02.txt"))

(define (tonum move)
  ; convert a letter literal to its number
  ; rock=0, paper=1, scissors=2
  (cond ((or (equal? move "A") (equal? move "X")) 0)
        ((or (equal? move "B") (equal? move "Y")) 1)
        (else 2)))

(define (main moves scorer)
  (if (null? moves) 0
    (+ (scorer (map tonum (car moves)))
       (main (cdr moves) scorer))))

(define (pt1scorer throw)
  ; throw: pair of numerated throws in a round / line of file, e.g. (0 1)
  ; modulo input: opponent - self
  ; modulo result: 2 -> 6, 1 -> 0, 0 -> 3
  (+ 1
     (cadr throw)  ; throw number + 1 gives value of the throw
     (case (modulo (apply - throw) 3)
       (2 6)
       (1 0)
       (else 3))))

(define (pt2scorer throw)
  ; throw: pair of numerated throws in a round / line of file, e.g. (0 1)
  ; modulo input: opponent + win/lose - 1
  ; modulo output: value of own throw
  (+ 1
     (modulo (+ (apply + throw) -1) 3)  ; calculate the value of own throw
     (* (cadr throw) 3)))

(display (main input pt1scorer))
(newline)
(display (main input pt2scorer))
(newline)
