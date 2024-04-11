# wn

wn is a version control system, inspired by git, created as an experiment for myself. Powered by Rust.

It is not meant to replace or be compatible with git, but it works very similarly.

wn stands for "why not", as in "why not make another version control system"?

## Goals

_v0.1_

- [x] Create a repository through wn
- [ ] Commit files
- [ ] Store compressed files from commit
- [ ] Link to previous versions for unchanged files
- [ ] Create branches
  - [ ] Allow detached HEAD
- [ ] Revert commits
- [ ] Merge branches
  - [ ] Squishing
  - [ ] Zip merge (all commits from branch B are filed into branch A)
- [ ] Track wn through itself

_v0.2_

- [ ] Compress repository into a smaller file (with deltas) to send to a server
- [ ] Simple server to accept changes
- [ ] Staging area to not commit directly

_v0.3_

- [ ] Automatically compress local repository into smaller files with delta if too big.

## How it will work

_For now, this is hypothetical. Actual implementation is being developed_

When the repository is first created, the following structure is made:

```
.wn/
  refs/
    main
  objects/
```

The main file for now will be empty.

### Creating a commit

Let's say we run the command

```bash
echo -n 'version 1' > file.txt
wn add file.txt
wn commit -m "Initial commit"
```

We'll now have a directory that looks like such:

```
.wn/
  refs/
    main
  objects/
    2b/
      8a1e4be95f0d7e9736222d21ab6efb8298c6b8e01cb2d44dba17fa9a355e83
    93/
      678739589f9e6ae099722299ffe32bf24e943bccfe30d92decbe743fb335d4
    d0/
      5e8bf1b1daaa7ecc2268b908ddb1499f20a3c3af084065b554a28873504375
```

`main` been modified, and now looks like this:

```
93678739589f9e6ae099722299ffe32bf24e943bccfe30d92decbe743fb335d4
```

This sha256 hash points to our new commit object:

```
commit 2b8a1e4be95f0d7e9736222d21ab6efb8298c6b8e01cb2d44dba17fa9a355e83
author Jacob
email example@example.com
time 1712553461
```

In turn, this sha256 hash refers to the file in the objects directory, which in turn looks like this:

```
dir 1
file file.txt d05e8bf1b1daaa7ecc2268b908ddb1499f20a3c3af084065b554a28873504375
```

The first line defines it as a directory (git terminology would be a `tree`) along with the number of entries in this directory.

This file's hash resolves to another object file that looks like this:

```
file 9
version 1
```

The first line defines it as a file (git would call this a `blob`) and lists the file size. Then the file content.

Most file contents will be compressed with zlib.

### Manually hash

You can run all this logic manually, for example:

```bash
$ echo -n 'version 1' > file.txt
$ wn object add file file.txt
d05e8bf1b1daaa7ecc2268b908ddb1499f20a3c3af084065b554a28873504375
$ wn object add tree file.txt
2b8a1e4be95f0d7e9736222d21ab6efb8298c6b8e01cb2d44dba17fa9a355e83
$ wn object add commit 2b8a1e4be95f0d7e9736222d21ab6efb8298c6b8e01cb2d44dba17fa9a355e83 --author "Jacob" --email example@example.com --time 1712553461
93678739589f9e6ae099722299ffe32bf24e943bccfe30d92decbe743fb335d4
```

## License

Licensed under Apache License 2.0. Contributions are welcome

## Credit

<a href="https://github.com/jacobmacweb/wn/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=jacobmacweb/wn" />
</a>

Made with [contrib.rocks](https://contrib.rocks).
