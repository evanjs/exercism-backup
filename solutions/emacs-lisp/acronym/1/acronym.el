;;; acronym.el --- Acronym (exercism)  -*- lexical-binding: t; -*-

;;; Commentary:

;;; Code:


(defun acronym (phrase)
  "Split string PHRASE using `split-string-and-unquote`.
Use `\s`, `_`, and `-` as separator characters.
Then, collect the first letter of each split and return the result.
Make sure the result is in ALL CAPS. _Not_ title case (i.e. )"

  (upcase
   (mapconcat
    (lambda (s)
      (substring s 0 1))
    (split-string-and-unquote phrase "[\s\_-]"))))

(provide 'acronym)
;;; acronym.el ends here
