# Counting Contract - CW Academy learning project

This is the counting contract implementation, with one commit per lesson to
make it easier to follow it. It is created by following the [CosmWasm
academy](academy.cosmwasm.com) Smart Contracts basics course step by step.

## Contributing

This repository is meant to have a very structured git history - single commit
per lesson/assignment, to allow single referring to particular points. Because
of that, any merges or pull requests of code would not be possible, as it would
break this structure. Any change besides things not directly course-related
(CI, markdowns, licensing, etc.) should be performed by going back to the
relevant lesson commit, amending it, and then rebasing the whole following
codebase to a new commit, force-pushing to the main branch at the end. It is a
terrible idea in practice to work like that in the typical codebase. Still, the
purpose of this particular codebase is not to deliver the product but to give a
learning resource to follow.

Conceding that, if you found some inconsistency between a course and the repo,
create an issue, and possibly even propose a solution, but do not expect it
would be merged from there - the repo maintainer would have to do it the dirty
way.
