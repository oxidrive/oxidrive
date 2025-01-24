insert into files (id, owner_id, name, content_type, size, tags) values
    (
        '019433e9-ffbb-7c8b-af6c-d4cb061fb919',
        '0194327d-becc-7ef3-809c-35dd09f62f45',
        'hello.txt',
        'text/plain',
        0,
        json('{ "name": "hello.txt", "content_type": "text/plain", "ext": "txt", "file1": {} }')
    ),
    (
        '019433ea-5976-7982-bedb-760ad14d4c1a',
        '0194327d-becc-7ef3-809c-35dd09f62f45',
        'world.txt',
        'text/plain',
        0,
        json('{ "name": "world.txt", "content_type": "text/plain", "ext": "txt", "file2": {} }')
    )
;
