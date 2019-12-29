# Writing Filesystem Changes
Instead of immediately writing to destination files, we need some way to rollback changes in case of partial failure.

So we probably need to move in stages:
1. Create the changed files.
2. Move the original files to temp files.
3. Move the changed files to the original paths.
    1. If any one of the moves fails, revert all of the successful moves and stop here.
4. Delete the temp files.

Now we have one possible failure point: if (3.1) fails and a successfuly-moved file can't be reverted. Possible reasons for such a failure include:
1. Access to the path is lost (USB drive ejected, Box disconnected)
2. *Privilege* for the path is lost:
    1. Someone opened the path while we weren't using it?
    2. We're no longer the owner of the file (someone deleted the path and then created a new file while we weren't using it)

Both cases of (2) aren't really workable: in one case the file is locked and we don't know when it'll be opened again, while in the other we outright no longer have permission to operate on the file. We can tell the user which files can't be reverted, but it's up to them to resolve the issue by unlocking those files.

Case (1) is workable from our end, however. We still have the temp files and can do one of two things:
1. Let the user open the temp as a backup, then choose whether or not to overwrite the existing file
2. Restore *all* of the temp files

(1) is probably the better option, since it's possible for (2) to fail and bring us back to our initial problem.