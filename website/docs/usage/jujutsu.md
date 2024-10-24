---
sidebar_position: 12
---

# Jujutsu

You can use with a repository that has been cloned using [jujutsu](https://martinvonz.github.io/jj/latest/).

## Colocated

If the repository was cloned by `jujutsu` using the `--colocate` option, then all you need to do is make sure that
you have checked out your mainline branch using git.
If you don't, then you will likely see an error about an unborn branch.

## Non-colocated

If the repository was cloned by `jujutsu` but *not* using the `--colocate` option,
then the Git repository, normally the `.git` directory, is located in `.jj/repo/store/git`

You can do one of two things to make your repository compatible with `git-cliff`:

### Fake Colocation

Create a file in the root of your repository that tells Git, and `git-cliff` where the Git repository is:

```bash
echo "gitdir: .jj/repo/store/git" > .git
```

### Remote branches

Update the `HEAD` of the Git repository to point to your remote mainline branch.

```bash
echo "ref: refs/remotes/origin/main" > .jj/repo/store/git/HEAD
```

N.B. Replace `main` with the name of the remote branch you want to use. e.g. `master`, `trunk`, `mainline`, etc.

