collections-title = Collections

collections-files-count = { $count -> 
        [one] { $count } file
        *[other] { $count } files
    }