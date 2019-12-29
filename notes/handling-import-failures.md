# Handling Import Failures
The notes on disk might fail to load into memory for various reasons:
* The file is inaccessible
* The file is accessible, but isn't deserializable

In both these cases, this shouldn't affect the data on disk. But since the node may cause the same error on loading later, we may want to give users the option to delete that node:
1. Remove the node from its container's list
2. Update the container's list on disk
3. Delete the node's disk implementation - this means deleting its file or its directory, depending on the node type

If (3) fails, that's sort of okay, since the container won't try to open the node anyway. But it's an issue if the user tries to create a node there again; anything there needs to be overwritten, and if there's any failures to write, the entire node is considered failed. This needs to be handled somehow.