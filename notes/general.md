# General Notes
## Workspace Layout
* We can generally follow the concepts demonstrated by Quiver:
    * Library Structure
        * A library containing notebooks, which each contain notes.
            * Each node in this - the library, the notebooks, the *notes* - **they're *all* directories** with a ```meta.json``` describing the node's metadata and relation to other elements in the library.
                * This appears to be why Quiver freaks out, as some filesystem properties aren't updating right for one or more directories. *Specifically* the directories, none of the JSON files seem to have filesystem issues. A ZIP file wouldn't have these kind of issues, but might have problems syncing just the new changes rather than the whole file.
                * Presumably this is where Quiver puts its tagging system too.
        * The directory system works just fine; we're not supporting any collab features as Quiver does.
        * For exporting, we could export to a ZIP/RAR.
    * Note File Format
        * .qvnotes are in fact *directories*. They always contain a ```meta.json``` and a ```content.json```; for this section we care about the ```content.json```.
        * The content file consists of:
            * A dictionary
                * Key "title": A string for the note's title.
                * Key "cells": An array of dictionaries for each cell in the note, starting with the topmost cell and going down.
                    * Key "type": What kind of cell this is. Types include "text", "markdown", "code", and more.
                    * Key "data": The data contained by the cell. Usually text, but for an image it'd probably have to either be a relative link or the actual content byte64'd or something.
                    * Key "language": If type == "code", this is the language the cell's using to highlight data.
    * The following "general remarks" section has a bunch of useful information that's not redundant to this section. **In particular, it mentions some security issues to consider**, so look at the section.
* Note Structure
    * A single window has a single **workspace** loaded.
        * A workspace has zero or more **notebooks**.
            * A notebook has zero or more **notes**.
                * A note is composed of zero or more **sections**, which each have a representation format, content, and index order in the note.
* Note Navigation
    * Generally workspaces are switched by using the Open command.
        * Notebooks are switched by clicking on the notebook in the window's notebook list.
            * Notes are switched by clicking on the note in the window's note list.
                * Section navigation is unclear, but most likely they're switched by clicking on the section in a section list.
    * Questions
        * When should a new window be opened? An "Open in New Window" makes sense, but should "Open" itself open in a new window under any circumstances?

## Possible File Format
* Individual Notes
    * Format
        * Entire file should be Unicode text (UTF16?).
        * Should be JSON or a similar plaintext structural format.
    * Structure
        * Should have a section for:
            * Meta tags
            * Raw note content
                * Should be Markdown
* Workspace
    * Format
        * Unicode text again.
        * Structure can be JSON, but note data can vary:
            * Maybe all the notes could be stored as a ZIP file, then live unlocked? Then periodically the file's updated that way.
                * **Security issues...**
                * Problem is that the entire file needs to be written on changes. We'd like a compression format that works on sections of the file.
                * But for security, we'd want to encrypt the entire note space so attackers can't even know the workspace structure.
                * This is probably a solved problem, ask engineering.
    * Structure
        * Workspace metadata
            * What method is the workspace compressed with, if any?
            * What method is the workspace encrypted with, if any?
            * Has the workspace been unlocked?
        * Workspace content. All data here is encrypted and compressed as per metadata.
            * All tags
            * Note filesystem:
                * A tree of directories and notes.
    * Validation of workspace
        * If a workspace-global list of all tags is stored, then it must be validated against all notes.
            * If validation fails, we need to regenerate the global list.
        * Tree might need to be rebuilt if a write failed?
            * How would I even do that, though?

## Handling File Events
* What can a file do?
    * Open
        * Start opening
        * Finish opening
    * Close
        * Start closing
        * Finish closing
    * Be read
        * Start reading
        * Finish reading
    * Be written
        * Start writing
        * Finish writing
    * Notify updates
        * Finish updating
            * There's no 'start updating' since from our client perspective we don't know when an update is begun.
* So any version of the app must handle all of these events in some way (or specify that they do nothing).
* In particular, failure cases must be handled:
    * A file's locked, so it can't be opened/written.
    * Permission's denied.
    * No space on disk (though native dialogs may handle much of this).
    * A file no longer exists after being opened! (Box dismounts on daemon close, for instance).

## App Version 1

## App Version 2
Same as 1, except:
* Notify updates
    * Finish updating
        * Update the app workspace.
        * If a file isn't open, update it.
        * If a file is open but isn't changed, update it.
        * Else, mark in the window/tab that the file's changed on disk.