(
 (nil . ((projectile-project-compilation-cmd . "ninja -C _build")
         (projectile-project-test-cmd . "ninja -C _build test")
         (projectile-project-configure-cmd . "meson . _build")
         (projectile-project-compilation-dir . ".")
         (projectile-project-run-cmd . "G_MESSAGES_DEBUG=all _build/examples/call-ui-demo")))
 (c-mode . (
            (c-file-style . "linux")
            (indent-tabs-mode . nil)
            (c-basic-offset . 2)
            ))
 (setq auto-mode-alist (cons '("\\.ui$" . nxml-mode) auto-mode-alist))
 (nxml-mode . (
            (indent-tabs-mode . nil)
            ))
 (css-mode . (
            (css-indent-offset . 2)
            ))
)

