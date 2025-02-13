insert into files (id, owner_id, name, content_type, size, tags, hash) values
    (
        '019433e9-ffbb-7c8b-af6c-d4cb061fb919',
        '0194327d-becc-7ef3-809c-35dd09f62f45',
        'hello.txt',
        'text/plain',
        0,
        json('{ "name": "hello.txt", "content_type": "text/plain", "ext": "txt", "size": "0", "file1": {} }'),
        x'd74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24'
    ),
    (
        '019433ea-5976-7982-bedb-760ad14d4c1a',
        '0194327d-becc-7ef3-809c-35dd09f62f45',
        'world.txt',
        'text/plain',
        0,
        json('{ "name": "world.txt", "content_type": "text/plain", "ext": "txt", "size": "0", "file2": {} }'),
        x'd74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24'
    )
;
