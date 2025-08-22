PUT http://127.0.0.1:3000/gutp/v1/user/update
[Form]
id: {{user_id}}
account: owner_user_123_updated
oauth_source: github
nickname: Updated Subspace Owner
avatar: https://example.com/updated_owner.png

HTTP 200
[Asserts]
jsonpath "$[0].account" == "owner_user_123_updated"
