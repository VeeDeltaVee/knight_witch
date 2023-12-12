# Contributing
Thanks for thinking of contributing to `knight_witch`!

## Hosting
This project is dual hosted on [Github](https://github.com/VeeDeltaVee/knight_witch)
and [Sourcehut](https://sr.ht/~lady_interstellar/knight_witch/). You can
contribute on either, depending on what you prefer.

### Github
Github follows a pull request workflow. You can contribute by forking the
repository in Github, committing your changes, and opening a pull request. For
more information, see the [Github Docs](https://docs.github.com/en/get-started/quickstart/contributing-to-projects)

### Sourcehut
Sourcehut follows an email based workflow. You have the following options for
submitting changes:

1. If you're new to the git email workflow, or are coming from a Github-like
   pull request workflow, sourcehut provides an experience that will be familiar
   You can follow this [documentation](https://man.sr.ht/git.sr.ht/send-email.md)
   on how to use these tools to submit a patch. There's also a video tutorial
   [here](https://spacepub.space/w/ad258d23-0ac6-488c-83fc-2bacf578de3a).
2. If you're familiar with how `git send-email` works, and have worked with an
   email setup before, we accept patches that way! If you are new and would like
   to learn how to submit patches this way, please follow this [handy
   guide](https://git-send-email.io/) Just send patches to the following mailing
   list: ~lady_interstellar/knight_witch-devel@lists.sr.ht.
3. If your change set is really large, say over a few hundred lines, a patch set
   might be difficult to follow. In this case, you can use `git request-pull` to
   submit your changes. Please follow documentation on the
   [Git docs](https://git-scm.com/docs/git-request-pull) on more information on how
   to do that. In general though, please first send a proposal on the mailing list
   before working on changes this big.

## Guidelines
Whichever way you chose to contribute, there's a few guidelines to follow:

1. For your commit messages, patch cover letters, and pull request
   summary/descriptions, please follow the [Conventional Commits](https://www.conventionalcommits.org)
   guidelines when making your commits. In addition, in general the titles
   shouldn't exceed 50 characters, and the descriptions should be wrapped at 72
   lines, as per git conventions.
2. Please ensure your git history is "clean". Feel free to follow whatever
   workflow works for you while you're working on your changes, but before
   submitting your changes please `rebase` or `git commit --amend` your changes
   into nice, atomic clean commits. If you recieve feedback, please don't make a
   new commit, instead change your history as if the feedback was incorporated
   from the start. This makes for easier reviewing and results in a cleaner
   final commit history.
3. Ensure that your changes are made up of atomic commits, where each commit
   compiles and has all tests passing.
4. Make mistakes! It's ok to send a malformed patch, or a PR without completely
   sticking to the above guidelines. It might be that you're on an edge case
   where it actually does make sense to break the guidelines, or maybe you're
   not sure what to do. In either case, as long as you're receptive to feedback
   and are willing to make changes, any and all contributions are welcome!
   Please do not hesitate to fire off a request.
