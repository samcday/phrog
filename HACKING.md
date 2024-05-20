Building
========
For build instructions see the README.md

Pull requests
=============
Before filing a pull request run the tests:

```sh
ninja -C _build test
```

Use descriptive commit messages, see

   https://wiki.gnome.org/Git/CommitMessages

and check

   https://wiki.openstack.org/wiki/GitCommitMessages

for good examples.


Coding Style
============
The code style used here is heavily inspired/copied from [phosh's][1]
and [calls'][2] Coding style.


g_assert() vs g_return_if_fail()
============
`g_assert ()` should only be used in internal checks. For the public API
you should guard against wrong parameters with `g_return_if_fail ()`.
