; text file reading helpers for plt-r5rs from Dr. Racket

(define (read-file-as-list filename)
  ; reads in a file given by filename as a list of strings
  (let ((file (open-input-file filename)))
    (define (loop line lines)
      ; build up list of lines, line is current line
      (let ((char (read-char file)))
        (if (eof-object? char)
            (reverse lines)
            (if (char=? char #\newline)
                (loop "" (cons line lines))
                (loop (string-append line (string char)) lines)))))
      (loop "" '())))

(define (split-string s delim)
  ; split a string on delim into a list of strings
  (define (loop curword line words)
    ; build up curword from line, then put into list of words
    ; (display (list curword line words))
    ; (newline)
    (if (= (string-length line) 0)
        (reverse (cons curword words))
        (let ((newchar (substring line 0 1))
              (lineleft (substring line 1)))
            (if (equal? delim newchar)
                (loop "" lineleft (cons curword words))
                (loop (string-append curword newchar)
                      lineleft
                      words)))))
  (loop "" s '()))

(define (split-string-space s)
  (split-string s " "))

(define (read-file-split-spaces filename)
  (map split-string-space (read-file-as-list filename)))
