
(define-module (parser stis-parser examples calc)
  #:use-module (ice-9 match)
  #:use-module (parser stis-parser)
  #:use-module (parser stis-parser operator-parser)
  #:export (p e c p2))

(define ws (f* (f-or! f-nl (f-char #\space) (f-char #\tab))))

;;;; let's define whitespace as a sequence fo nl/space/tab
(fluid-set! *whitespace* ws)

    ;(f-or! a b ...) try first a if that fails b ... ! means only one solution

;;;;Define the tokenizer

;;number
(define int (f+ (f-reg! "[0-9]")))
                                        ; ! means store character
                                        ; f+ mean 1 or more matches
                                        ; f-reg mean that the character
                                        ; should match the regular expression
                                        ; for one character
                                        ; does not work for multiple characters

(define decimal (f+ (f-or! (f-seq int (f-tag! ".") int)
                           (f-seq int (f-tag "."))
                           (f-seq (f-tag! ".") int)
                           int)))
                                        ;! mean store match
                                        ;f-seq a ... means match a, then b ...
                                        ;f-tag means literal match of all
                                        ;characters in string

(define exp (f-seq decimal (f-reg! "[eE]") (f? (f-reg! "[+-]")) decimal))
(define num (f-or! exp decimal))

;; lets make a token
(define number-
  (p-freeze 'number (mk-token num)
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

;;symbol ; nothing new just a c-symbol
(define sym (f-seq (f-reg! "[_a-zA-Z]") (f* (f-reg! "[_a-zA-Z0-9]"))))
(define symbol-
  (p-freeze 'symbol (mk-token sym)
    (lambda (s cin cout)
      (string->symbol cout))))
(define symbol (f-list #:symbol symbol-))

(define comma (f-cons* (D add/sub-expr) (ff* (f-seq "," (D add/sub-expr)))))
(define fkn   (f-cons* #:fkn symbol (f-seq "(" comma ")")))
                                        ;f-cons a ... b will cons* the ouput
                                        ;of a .... b
                                        ;ff+ and ff* creates lists of the
                                        ;results

;; paranthesizes expressions, note use (D f) when f is not defined yet
(define sexpr    (f-seq (f-tag "(") (D add/sub-expr) f-tag ")"))
(define term     (f-or! sexpr fkn symbol number))

(define term-1   (f-or! (f-list #:+ "+" (D term-1))
                        (f-list #:- "-" (D term-1))
                        term))
                                        ;note that strings will sielently match
                                        ;it's value

(define eql-expr (f-list #:= symbol "=" (D add/sub-expr)))
(define pow-expr (f-or! (f-cons* #:^ term-1 (ff+ (f-seq "^" term-1)))
                        term-1))

(define mul-expr (f-list #:* pow-expr "*" (D mul/div-expr)))
(define div-expr (f-list #:/ pow-expr "/" (D mul/div-expr)))
(define mul/div-expr (f-or! (f-or! mul-expr div-expr)
                            pow-expr))

(define add-expr (f-list #:+ mul/div-expr   "+" (D add/sub-expr)))
(define sub-expr (f-list #:- mul/div-expr   "-" (D add/sub-expr)))
(define add/sub-expr (f-or! add-expr
                            sub-expr
                            mul/div-expr))



(define hexpr (f-list #:aab "abcd"))
(define expr (f-or! hexpr eql-expr add/sub-expr))
;(define expr (f-or! eql-expr add/sub-expr))

;;;;Now lets define the parser
(define (p string) ((@ (parser stis-parser) parse)
                    string (f-seq expr f-eof)))

                                        ;we must end with f-eof e.g.
                                        ;end of string in order to not parse
                                        ;a prefix

;;;; Voila
;; cheme@(guile-user)> (use-modules (parser stis-parser examples calc))

(display (p "1234 + 4^-3*12-3/2 - a")) (newline)
(newline)
(display (p "abcd"))(newline)
(newline)



;;scheme@(guile-user)> (calc-parse "1234 + 4^-3*12-3/2 - a")
;;(#:= (#:symbol b)
;;     (#:+ (#:number 1234)
;;          (#:- (#:* (#:^ (#:number 4)
;;                         (#:- (#:number 3)))
;;                    (#:number 12))
;;               (#:- (#:/ (#:number 3) (#:number 2))
;;                    (#:symbol a)))))
            
(define (e str)
  (let ((mod (current-module)))
    (let lp ((r (p str)))
      (match r
        ((#:= (#:symbol s) expr)
         (let ((e (lp expr)))
           (if (module-defined? mod s)             
               (module-set! mod s (lp expr))
               (module-define! mod s (lp expr)))
           e))
        ((#:+ a)
         (lp a))
        ((#:symbol s)
         (module-ref mod s))
        ((#:number n)
         n)
        ((#:fkn (#:symbol s) . a)
         (eval (cons s (map lp a)) mod))
        ((#:- a)
         (- (lp a)))
        ((#:+ a b)
         (+ (lp a) (lp b)))
        ((#:- a b)
         (- (lp a) (lp b)))
        ((#:* a b)
         (* (lp a) (lp b)))
        ((#:/ a b)
         (/ (lp a) (lp b)))
        ((#:^ a b)
         (expt (lp a) (lp b)))
        ((#:^ a . b)
         (expt (lp a) (lp (cons #:^ b))))))))

(define (c str)
  (let lp ((r (p str)))
    (match r
      ((#:= (#:symbol s) expr)
       (let ((x (gensym "x")))
         `(let ((,x ,(lp expr)))
            (define! ',s ,x)
            ,x)))
      
      ((#:+ a)
       (lp a))
      
      ((#:symbol s)
       s)
      
      ((#:number n)
       n)
      
      ((#:fkn (#:symbol s) . a)
       (cons* s (map lp a)))
      ((#:- a)
       `(- ,(lp a)))
      ((#:+ a b)
       `(+ ,(lp a) ,(lp b)))
      ((#:- a b)
       `(- ,(lp a) ,(lp b)))
      ((#:* a b)
       `(* ,(lp a) ,(lp b)))
      ((#:/ a b)
       `(/ ,(lp a) ,(lp b)))
      ((#:^ a b)
       `(expt ,(lp a) ,(lp b)))
      ((#:^ a . b)
       `(expt ,(lp a) ,(lp (cons #:^ b)))))))


;;with the compiler c to scheme you can now do

#|
scheme@(guile-user)> ,L calc
Happy hacking with Calc!  To switch back, type `,L scheme'.
calc@(guile-user)> x=1
$1 = 1
calc@(guile-user)> x+2
$2 = 3
calc@(guile-user)> pi=3.13
$3 = 3.13
calc@(guile-user)> pi
$4 = 3.13
calc@(guile-user)> sin(pi)
$5 = 0.011592393936158275
calc@(guile-user)> sin(pi/2)
$6 = 0.9999832013448761
calc@(guile-user)> sin(pi/3)
$7 = 0.8640868338458068
calc@(guile-user)> sin(pi/3)^10
$8 = 0.2320458886621503
calc@(guile-user)> y=sin(pi/3)^10
$9 = 0.2320458886621503
calc@(guile-user)> y
$10 = 0.2320458886621503
calc@(guile-user)> 
|#


;;;; Let's checkout the expression dynamic parser here you define a set
;;;; of operators and some rules and get a parser emmediately out of it
;;;; you can even dynamically create operators :-)

  
(define *ops* (make-opdata))

(for-each
 (lambda (x)
   (match x
     ((a b c) (add-operator *ops* a c b ws))))
 `((xfy 50 ",")
   (xfy 30 +) ; binary operators l-r
   (xfy 30 -)
   (xfy 20 *)
   (xfy 20 /)
   (yfx 10 ^) ; right to left expression
   (xfx 40 =) ; only a binary expression
   (fy  5  +) ; unary postfix operators
   (fy  5  -)))
#|
xfy = left to right
yfx = right to left
xfx = binary expression with just two terms
xf  = prefix operator
fy  = postfix operator

the numbers are the binding strength lower binds harder
|#


(define fkn2     (f-cons* #:fkn symbol (f-seq "(" (D expr2) ")")))
(define sexpr2   (f-seq (f-tag "(") (D expr2) (f-tag ")")))
(define term2    (f-or! sexpr2 fkn2 symbol number))

(define expr2 ((mk-operator-expression ws term2 f-false *ops*) 50))
                                        ; 50 is highest level
                                        ; ws = whitespace
                                        ; term2 = the term
                                        ; f-false expert option leave as ity is
                                        ; *ops* the generated operator table

(define (p2 string) ((@ (parser stis-parser) parse)
                     string (f-seq expr2 f-eof)))

#|
scheme@(guile-user)> (p2 "1^2^3")
$1 = ((yfx 10 "^" #\^) 
           ((yfx 10 "^" #\^) 
            (#:number 1) 
            (#:number 2) 
            3 1) 
      (#:number 3) 
      5 1)

So the parse tree has format
Binary = (Tag Term Term Line column)
UNARY  = (Tag Term Line column)
Tag    = (type level operator first-char)

With this one can deduce a simple evaluator compiler etc using e.g. 
C semantics
|#

