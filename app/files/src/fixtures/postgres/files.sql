insert into files (id, owner_id, name, content_type, size, tags, hash) values
    (
        '019433e9-ffbb-7c8b-af6c-d4cb061fb919',
        '0194327d-becc-7ef3-809c-35dd09f62f45',
        'hello.txt',
        'text/plain',
        0,
        'name => hello.txt, content_type => text/plain, file1 => null, size => 0, ext => txt'::hstore,
        decode('d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24', 'hex')
    ),
    (
        '019433ea-5976-7982-bedb-760ad14d4c1a',
        '0194327d-becc-7ef3-809c-35dd09f62f45',
        'world.txt',
        'text/plain',
        0,
        'name => world.txt, content_type => text/plain, file2 => null, size => 0, ext => txt'::hstore,
        decode('d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24', 'hex')
    )
;
