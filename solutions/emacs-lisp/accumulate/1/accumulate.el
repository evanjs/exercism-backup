;;; accumulate.el --- Accumulate (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:

(defun accumulate (lst op)
  "Recursively run the function OP against the list LST."
  (when lst
    (cons (funcall op (car lst))
          (accumulate (cdr lst) op))))

(provide 'accumulate)
;;; accumulate.el ends here
