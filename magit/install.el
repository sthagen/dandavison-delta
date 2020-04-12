;; -*- lexical-binding: t -*-

(defconst magit-delta--dependencies
  '(dash libgit transient with-editor xterm-color))

(defun magit-delta--package-install (package)
  (if (package-installed-p package)
      (message "%S: OK" package)
    (progn
      (package-install package)
      (message "%S: ...OK" package))))

(package-initialize)
(setq package-archives '(("melpa" . "https://melpa.org/packages/")))
(package-refresh-contents)
(mapc #'magit-delta--package-install magit-delta--dependencies)
