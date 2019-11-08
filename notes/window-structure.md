# Window Structure
* App should be one window. The window should have:
    * A list on the left of available notes
        * There should be a search bar to allow searching by name/tag/content.
    * The currently selected note's contents in the right side, filling available space.
        * The note editor should have two available modes:
            * Edit-only
                * The editor view is a single unsplit view listing the note contents as text nodes. Should contain:
                    * On the top, a toolbar for text node options. Can be hidden.
                    * On the bottom filling available, the text editor. You'll need some way to separate sections, since they'll each use a different renderer.
            * Edit with Preview
                * The editor view is split in half. The left half is the normal edit-only view, the right half is a render of the note.
* You should switch between workspaces with an "Open" command.