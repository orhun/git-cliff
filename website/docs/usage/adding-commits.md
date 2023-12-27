---
sidebar_position: 8
---

# Adding custom commits

In some cases, you might want to include commit messages in the changelog that yet don't exist. One example would be having "the commit message that updates the changelog" in the changelog. (🤔)

```bash
git cliff -o CHANGELOG.md
git add CHANGELOG.md
git commit -m "chore(release): update CHANGELOG.md for 1.0.0"
```

In the example above, `CHANGELOG.md` will not have the latest commit message since the commit is created afterward. So if you want to include custom commit messages like that in the changelog, you can use the `--with-commit` argument as follows:

```bash
# define the commit message
commit_msg="chore(release): update CHANGELOG.md for 1.0.0"

# generate changelog and pretend a commit exists as "$commit_msg"
git cliff --with-commit "$commit_msg" -o CHANGELOG.md

# create the actual commit
git add CHANGELOG.md
git commit -m "$commit_msg"
```

The commit SHA will be empty as default when `--with-commit` is used. Specify the hash with a message separated by single whitespace for setting the commit SHA. e.g. `--with-commit "8f55e69eba6e6ce811ace32bd84cc82215673cb6 feat: add X"`
