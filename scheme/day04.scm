(load "scheme/fileread.scm")  ; file reading helpers

(define input (read-file-as-list "inputs/day04.txt"))

(define (line-to-nums line)
  (let ((nums (map (split-string-fn "-") ; split on dashes, now nested lists
                     ((split-string-fn ",") line)))) ; split on middle comma
    (map string->number (list (caar nums) (cadar nums) (caadr nums) (cadadr nums)))))

(define (full-overlap? x) ; x is list of 4 numbers
  (let ((a (car x)) (b (cadr x)) (c (caddr x)) (d (cadddr x)))
    (or (and (>= a c) (<= b d))
        (and (<= a c) (>= b d)))))

(define (partial-overlap? x) ; x is list of 4 numbers
  (let ((a (car x)) (b (cadr x)) (c (caddr x)) (d (cadddr x)))
    (or (and (>= a c) (<= a d))
        (and (>= b c) (<= b d))
        (and (>= c a) (<= c b)))))

(define (score rem overlap?)
  (if (null? rem) 0
      (+ (if (overlap? (line-to-nums (car rem)))
             1 0)
         (score (cdr rem) overlap?))))

(display (score input full-overlap?))
(newline)
(display (score input partial-overlap?))
(newline)
