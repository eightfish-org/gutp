POST http://127.0.0.1:3000/gutp/v1/subspace/create
[Form]
title: space111
description: this is the subspace of test111
banner: hello world
owner_id: owner_id_1
subspace_id: subspace_id_1
is_public: true
slug: space_111
data_source: guforum

HTTP 200


POST http://127.0.0.1:3000/gutp/v1/post/create
[Form]
title: test111
content: this is the content of test111
author_id: my_id_1
subspace_id: subspace_id_1
ext_link: extlink_1
is_public: true
data_source: guforum
 
HTTP 200
[Captures]
post_id_1: jsonpath "$[0].id"


GET http://127.0.0.1:3000/gutp/v1/post?id={{post_id_1}}

HTTP 200


PUT http://127.0.0.1:3000/v1/post/update
[Form]
id: {{post_id_1}}
title: test111
content: this is the content of test222
author_id: my_id_1
author_nickname: my_nickname_1
subspace_id: subspace_id_1
ext_link: extlink_1
is_public: true
data_source: guforum

HTTP 200

POST http://127.0.0.1:3000/gutp/v1/post/create
[Form]
title: test222
content: this is the content of test222
author_id: my_id_2
subspace_id: subspace_id_2
ext_link: extlink_2
is_public: true
data_source: guforum
 
HTTP 200
[Captures]
post_id_2: jsonpath "$[0].id"


GET http://127.0.0.1:3000/gutp/v1/post?id={{post_id_2}}

HTTP 200


GET http://127.0.0.1:3000/gutp/v1/post/list

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$" count == 2
