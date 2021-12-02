#|
--- Day 2: Dive! ---

Now, you need to figure out how to pilot this thing.

It seems like the submarine can take a series of commands like forward
 1, down 2, or up 3:

  - forward X increases the horizontal position by X units.
  - down X increases the depth by X units.
  - up X decreases the depth by X units.

Note that since you're on a submarine, down and up affect your depth,
and so they have the opposite result of what you might expect.

The submarine seems to already have a planned course (your puzzle
input). You should probably figure out where it's going. For example:

forward 5
down 5
forward 8
up 3
down 8
forward 2

Your horizontal position and depth both start at 0. The steps above
would then modify them as follows:

  - forward 5 adds 5 to your horizontal position, a total of 5.
  - down 5 adds 5 to your depth, resulting in a value of 5.
  - forward 8 adds 8 to your horizontal position, a total of 13.
  - up 3 decreases your depth by 3, resulting in a value of 2.
  - down 8 adds 8 to your depth, resulting in a value of 10.
  - forward 2 adds 2 to your horizontal position, a total of 15.

After following these instructions, you would have a horizontal
position of 15 and a depth of 10. (Multiplying these together produces
150.)

Calculate the horizontal position and depth you would have after
following the planned course. What do you get if you multiply your
final horizontal position by your final depth?
|#

(write "Advent of Code Day 2!") (newline)

; https://gitlab.com/tampe/stis-parser

(use-modules (parser stis-parser))
(use-modules (ice-9 rdelim))
(use-modules (ice-9 match))

(define ws (f* (f-or! f-nl (f-char #\space) (f-char #\tab))))

;;;; let's define whitespace as a sequence fo nl/space/tab
(fluid-set! *whitespace* ws)

;;number
(define int (f+ (f-reg! "[0-9]")))
                                        ; ! means store character
                                        ; f+ mean 1 or more matches
                                        ; f-reg mean that the character
                                        ; should match the regular expression
                                        ; for one character
                                        ; does not work for multiple characters
;; lets make a token
(define number-
  ;(p-freeze 'number (mk-token num)
  (p-freeze 'number (mk-token int)
    (lambda (s cin cout)
      (string->number cout))))
                                        ;(mk-token f) will combine the result of
                                        ;f in one string
                                        ;(p-freeze tag f translate)
                                        ;well memoize the result of f tagging it
                                        ;with the uique tag tag and translate
                                        ;the result with (lambda (s in out) ..)
                                        ;most cases will use out
;;Tag the token e.g. produce a value '(#:number 2322.2e-122.2) we also have
;;  f-cons f-cons* that works naturally there is whitespace between the sub
;;  expressions
(define number (f-list #:number number-))


(define forward (f-list #:forward "forward"))
(define down (f-list #:down "down"))
(define up (f-list #:up "up"))

(define expr (f-list (f-or! forward up down) ws number))


(define (p string) ((@ (parser stis-parser) parse)
                    string (f-seq expr f-eof)))

;(display (p "forward 23")) (newline)
;(newline)

(define (move_from_file filename)
    (let
        ((port (open-input-file filename)))
        (display (get_dir_from_port port)) (newline)
        (close-port port)
    )
)



(define (get_dir_from_port theport)
        (let ((sline (read-line theport)))
        (if (eof-object? sline)
            0+0i
            (let ((dir_len (p sline)))
                (display dir_len) (newline)
                (let ((dir (car dir_len)) (len (cdr dir_len)))
                    ;(display dir) (display len) (newline)
                    ;(when (equal? dir '(#:forward)) (display "FORWARD EQUAL") (newline))
                    ;(when (eqv? dir '(#:forward)) (display "FORWARD EQV") (newline))
                    ;(when (eq? dir '(#:forward)) (display "FORWARD EQ") (newline))
                    ;(case dir
                    ;    ('(#:forward) '(4 4))
                    ;    (else '(1 1))
                    ;)
                    ;(cond
                    ;    ((equal? dir '(#:forward)) 4+5i)
                    ;)
                    (match dir_len 
                    ;(match len 
                        (((#:forward) (#:number n))
                            (+ n (get_dir_from_port theport))
                        )
                        (((#:down) (#:number n))
                            (- (get_dir_from_port theport) (* n 0+1i))
                            ;(get_dir_from_port theport)
                        )
                        (((#:up) (#:number n))
                            (+ (get_dir_from_port theport) (* n 0+1i))
                            ;(get_dir_from_port theport)
                        )
                        
                        ((#:number n)
                        n)
                        (((#:number n))
                        n)
                    )
                )
                )
        )))



(move_from_file "example.txt")
;(move_from_file "puzzle.txt")


