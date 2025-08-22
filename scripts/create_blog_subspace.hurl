POST http://127.0.0.1:3000/gutp/v1/subspace/create
[Form]
slug:  
title: {{title}}
description:   
banner:   
is_public: true
owner_id: {{user_id}}

HTTP 200
[Captures]
subspace_id: jsonpath "$[0].id"
[Asserts]
jsonpath "$[0].owner_id" == {{user_id}}

