# GUTP API Tests - Comprehensive Test Suite with Foreign Key Constraints
# Run with: hurl apitests.hurl
# This test suite respects all foreign key relationships and tests constraint violations

# ===== SETUP: CREATE BASE ENTITIES IN CORRECT ORDER =====

# Create base users first (no foreign key dependencies)
POST http://127.0.0.1:3000/gutp/v1/user/create
[Form]
account: owner_user_123
oauth_source: github
nickname: Subspace Owner
avatar: https://example.com/owner_avatar.png

HTTP 200
[Captures]
owner_user_id: jsonpath "$[0].id"
[Asserts]
jsonpath "$[0].account" == "owner_user_123"
jsonpath "$[0].role" == 0
jsonpath "$[0].status" == 0


POST http://127.0.0.1:3000/gutp/v1/user/create
[Form]
account: author_user_456
oauth_source: google
nickname: Content Author
avatar: https://example.com/author_avatar.png

HTTP 200
[Captures]
author_user_id: jsonpath "$[0].id"


POST http://127.0.0.1:3000/gutp/v1/user/create
[Form]
account: commenter_user_789
oauth_source: facebook
nickname: Active Commenter
avatar: https://example.com/commenter_avatar.png

HTTP 200
[Captures]
commenter_user_id: jsonpath "$[0].id"


# ===== USER MODULE TESTS =====

# Test getting users
GET http://127.0.0.1:3000/gutp/v1/user?id={{owner_user_id}}

HTTP 200
[Asserts]
jsonpath "$[0].id" == {{owner_user_id}}
jsonpath "$[0].account" == "owner_user_123"


# Test user uniqueness constraint
POST http://127.0.0.1:3000/gutp/v1/user/create
[Form]
account: owner_user_123  # Duplicate account
oauth_source: github
nickname: Duplicate User
avatar: https://example.com/dup.png

HTTP 400  # Should fail due to UNIQUE constraint on account


# Update user
PUT http://127.0.0.1:3000/gutp/v1/user/update
[Form]
id: {{owner_user_id}}
account: owner_user_123_updated
oauth_source: github
nickname: Updated Subspace Owner
avatar: https://example.com/updated_owner.png

HTTP 200
[Asserts]
jsonpath "$[0].account" == "owner_user_123_updated"


# ===== SUBSPACE MODULE TESTS =====

# Create subspace with valid owner (respects FK constraint)
POST http://127.0.0.1:3000/gutp/v1/subspace/create
[Form]
slug: tech-discussions
title: Technology Discussions
description: A place for tech enthusiasts
banner: https://example.com/tech_banner.png
is_public: true
owner_id: {{owner_user_id}}

HTTP 200
[Captures]
tech_subspace_id: jsonpath "$[0].id"
[Asserts]
jsonpath "$[0].owner_id" == {{owner_user_id}}


# Create subspace with NULL owner (owner_id is nullable)
POST http://127.0.0.1:3000/gutp/v1/subspace/create
[Form]
slug: orphan-subspace
title: Orphan Subspace
description: A subspace without an owner
banner: https://example.com/orphan_banner.png
is_public: true

HTTP 200
[Captures]
orphan_subspace_id: jsonpath "$[0].id"
[Asserts]
jsonpath "$[0].owner_id" == null


# Test creating subspace with non-existent owner (should fail)
POST http://127.0.0.1:3000/gutp/v1/subspace/create
[Form]
slug: invalid-owner-subspace
title: Invalid Owner Subspace
description: This should fail
banner: https://example.com/fail.png
is_public: true
owner_id: non_existent_user_id

HTTP 400  # Should fail due to FK constraint violation


# Create private subspace for testing
POST http://127.0.0.1:3000/gutp/v1/subspace/create
[Form]
slug: private-discussions
title: Private Discussions
description: Members-only content
banner: https://example.com/private_banner.png
is_public: false
owner_id: {{owner_user_id}}

HTTP 200
[Captures]
private_subspace_id: jsonpath "$[0].id"


# ===== POST MODULE TESTS =====

# Create post with valid author and subspace (respects both FK constraints)
POST http://127.0.0.1:3000/gutp/v1/post/create
[Form]
title: Introduction to WebAssembly
content: WebAssembly is a binary instruction format for a stack-based virtual machine.
author_id: {{author_user_id}}
subspace_id: {{tech_subspace_id}}
ext_link: https://webassembly.org
is_public: true

HTTP 200
[Captures]
post_id_1: jsonpath "$[0].id"
post_1_created_time: jsonpath "$[0].created_time"
[Asserts]
jsonpath "$[0].author_id" == {{author_user_id}}
jsonpath "$[0].subspace_id" == {{tech_subspace_id}}
jsonpath "$[0].parent_post_id" == null


# Test creating post with non-existent author (should fail)
POST http://127.0.0.1:3000/gutp/v1/post/create
[Form]
title: Invalid Author Post
content: This should fail
author_id: non_existent_author
subspace_id: {{tech_subspace_id}}
ext_link: https://example.com
is_public: true

HTTP 400  # FK constraint violation


# Test creating post with non-existent subspace (should fail)
POST http://127.0.0.1:3000/gutp/v1/post/create
[Form]
title: Invalid Subspace Post
content: This should fail
author_id: {{author_user_id}}
subspace_id: non_existent_subspace
ext_link: https://example.com
is_public: true

HTTP 400  # FK constraint violation


# Create nested post (with parent_post_id)
# POST http://127.0.0.1:3000/gutp/v1/post/create
# [Form]
# title: Re: Introduction to WebAssembly
# content: Great introduction! Here's a follow-up on WASM use cases.
# author_id: {{commenter_user_id}}
# subspace_id: {{tech_subspace_id}}
# parent_post_id: {{post_id_1}}
# ext_link: https://example.com/wasm-uses
# is_public: true

# HTTP 200
# [Captures]
# nested_post_id: jsonpath "$[0].id"
# [Asserts]
# jsonpath "$[0].parent_post_id" == {{post_id_1}}


# Create post in private subspace
POST http://127.0.0.1:3000/gutp/v1/post/create
[Form]
title: Confidential Project Update
content: This content should be encrypted in a private subspace.
author_id: {{owner_user_id}}
subspace_id: {{private_subspace_id}}
ext_link: https://internal.example.com/project
is_public: false

HTTP 200
[Captures]
private_post_id: jsonpath "$[0].id"


# ===== COMMENT MODULE TESTS =====

# Create comment with valid author and post (respects FK constraints)
POST http://127.0.0.1:3000/gutp/v1/comment/create
[Form]
content: Great article! WebAssembly is indeed revolutionary.
author_id: {{commenter_user_id}}
post_id: {{post_id_1}}
is_public: true

HTTP 200
[Captures]
comment_id_1: jsonpath "$[0].id"
[Asserts]
jsonpath "$[0].author_id" == {{commenter_user_id}}
jsonpath "$[0].post_id" == {{post_id_1}}
jsonpath "$[0].parent_comment_id" == null


# Test creating comment with non-existent author (should fail)
POST http://127.0.0.1:3000/gutp/v1/comment/create
[Form]
content: Invalid author comment
author_id: non_existent_user
post_id: {{post_id_1}}
is_public: true

HTTP 400  # FK constraint violation


# Test creating comment with non-existent post (should fail)
POST http://127.0.0.1:3000/gutp/v1/comment/create
[Form]
content: Invalid post comment
author_id: {{commenter_user_id}}
post_id: non_existent_post
is_public: true

HTTP 400  # FK constraint violation


# Create nested comment (reply)
POST http://127.0.0.1:3000/gutp/v1/comment/create
[Form]
content: I agree! WASM opens up so many possibilities.
author_id: {{author_user_id}}
post_id: {{post_id_1}}
parent_comment_id: {{comment_id_1}}
is_public: true

HTTP 200
[Captures]
comment_id_2: jsonpath "$[0].id"
[Asserts]
jsonpath "$[0].parent_comment_id" == {{comment_id_1}}


# Create deeply nested comment
POST http://127.0.0.1:3000/gutp/v1/comment/create
[Form]
content: Let me add to this discussion thread.
author_id: {{owner_user_id}}
post_id: {{post_id_1}}
parent_comment_id: {{comment_id_2}}
is_public: true

HTTP 200
[Captures]
comment_id_3: jsonpath "$[0].id"
[Asserts]
jsonpath "$[0].parent_comment_id" == {{comment_id_2}}


# Create comment on private post
POST http://127.0.0.1:3000/gutp/v1/comment/create
[Form]
content: Confidential feedback on the project update.
author_id: {{author_user_id}}
post_id: {{private_post_id}}
is_public: false

HTTP 200
[Captures]
private_comment_id: jsonpath "$[0].id"


# ===== CROSS-MODULE RELATIONSHIP TESTS =====

# List posts by existing author
GET http://127.0.0.1:3000/gutp/v1/post/list_by_author?author_id={{author_user_id}}

HTTP 200
[Asserts]
jsonpath "$" count >= 1
jsonpath "$[*].author_id" includes {{author_user_id}}


# List posts by existing subspace
GET http://127.0.0.1:3000/gutp/v1/post/list_by_subspace?subspace_id={{tech_subspace_id}}

HTTP 200
[Asserts]
# jsonpath "$" count >= 2
jsonpath "$" count == 1
jsonpath "$[*].subspace_id" includes {{tech_subspace_id}}


# List comments by existing post
GET http://127.0.0.1:3000/gutp/v1/comment/list_by_post?post_id={{post_id_1}}

HTTP 200
[Asserts]
jsonpath "$" count >= 3
jsonpath "$[*].post_id" includes {{post_id_1}}


# List subspaces by existing owner
GET http://127.0.0.1:3000/gutp/v1/subspace/list_by_owner?owner_id={{owner_user_id}}

HTTP 200
[Asserts]
jsonpath "$" count >= 2
jsonpath "$[*].owner_id" includes {{owner_user_id}}


# ===== FK CONSTRAINT DELETION TESTS (ON DELETE RESTRICT) =====

# Try to delete user who owns subspaces (should fail due to FK constraint)
DELETE http://127.0.0.1:3000/gutp/v1/user/delete
[Form]
id: {{owner_user_id}}

HTTP 400  # Should fail - user is referenced by subspaces


# Try to delete user who authored posts (should fail)
DELETE http://127.0.0.1:3000/gutp/v1/user/delete
[Form]
id: {{author_user_id}}

HTTP 400  # Should fail - user is referenced by posts


# Try to delete subspace with posts (should fail)
DELETE http://127.0.0.1:3000/gutp/v1/subspace/delete
[Form]
id: {{tech_subspace_id}}

HTTP 400  # Should fail - subspace is referenced by posts


# Try to delete post with comments (should fail)
DELETE http://127.0.0.1:3000/gutp/v1/post/delete
[Form]
id: {{post_id_1}}

HTTP 400  # Should fail - post is referenced by comments


# Try to delete comment that has child comments (should fail)
DELETE http://127.0.0.1:3000/gutp/v1/comment/delete
[Form]
id: {{comment_id_1}}

HTTP 400  # Should fail - comment is referenced by child comments


# ===== UPDATE TESTS WITH FK CONSTRAINTS =====

# Update post with valid new author
PUT http://127.0.0.1:3000/gutp/v1/post/update
[Form]
id: {{post_id_1}}
title: Introduction to WebAssembly (Updated)
content: Updated content with more details about WASM.
author_id: {{owner_user_id}}  # Change author to owner_user
ext_link: https://webassembly.org/docs
is_public: true

HTTP 200
[Asserts]
jsonpath "$[0].author_id" == {{owner_user_id}}
jsonpath "$[0].updated_time" > {{post_1_created_time}}


# ===== CLEANUP IN CORRECT ORDER (respecting FK constraints) =====

# First, delete leaf comments (no children)
DELETE http://127.0.0.1:3000/gutp/v1/comment/delete
[Form]
id: {{comment_id_3}}

HTTP 200


DELETE http://127.0.0.1:3000/gutp/v1/comment/delete
[Form]
id: {{private_comment_id}}

HTTP 200


# Delete comment that now has no children
DELETE http://127.0.0.1:3000/gutp/v1/comment/delete
[Form]
id: {{comment_id_2}}

HTTP 200


# Delete root comment
DELETE http://127.0.0.1:3000/gutp/v1/comment/delete
[Form]
id: {{comment_id_1}}

HTTP 200


# Now can delete posts (no more comments)
# DELETE http://127.0.0.1:3000/gutp/v1/post/delete
# [Form]
# id: {{nested_post_id}}

# HTTP 200


DELETE http://127.0.0.1:3000/gutp/v1/post/delete
[Form]
id: {{post_id_1}}

HTTP 200


DELETE http://127.0.0.1:3000/gutp/v1/post/delete
[Form]
id: {{private_post_id}}

HTTP 200


# Now can delete subspaces (no more posts)
DELETE http://127.0.0.1:3000/gutp/v1/subspace/delete
[Form]
id: {{tech_subspace_id}}

HTTP 200


DELETE http://127.0.0.1:3000/gutp/v1/subspace/delete
[Form]
id: {{private_subspace_id}}

HTTP 200


DELETE http://127.0.0.1:3000/gutp/v1/subspace/delete
[Form]
id: {{orphan_subspace_id}}

HTTP 200


# Finally, can delete users (no more references)
DELETE http://127.0.0.1:3000/gutp/v1/user/delete
[Form]
id: {{commenter_user_id}}

HTTP 200


DELETE http://127.0.0.1:3000/gutp/v1/user/delete
[Form]
id: {{author_user_id}}

HTTP 200


DELETE http://127.0.0.1:3000/gutp/v1/user/delete
[Form]
id: {{owner_user_id}}

HTTP 200
[Asserts]
jsonpath "$[0].id" == {{owner_user_id}}


# ===== ADDITIONAL EDGE CASES =====

# Test cascade behavior for self-referencing relationships
POST http://127.0.0.1:3000/gutp/v1/user/create
[Form]
account: test_cascade_user
oauth_source: github
nickname: Cascade Test User
avatar: https://example.com/cascade.png

HTTP 200
[Captures]
cascade_user_id: jsonpath "$[0].id"


POST http://127.0.0.1:3000/gutp/v1/subspace/create
[Form]
slug: cascade-test-space
title: Cascade Test Space
description: Testing cascade behaviors
banner: https://example.com/cascade_banner.png
is_public: true
owner_id: {{cascade_user_id}}

HTTP 200
[Captures]
cascade_subspace_id: jsonpath "$[0].id"


POST http://127.0.0.1:3000/gutp/v1/post/create
[Form]
title: Root Post for Thread Testing
content: This is the root of a discussion thread.
author_id: {{cascade_user_id}}
subspace_id: {{cascade_subspace_id}}
ext_link: https://example.com/root
is_public: true

HTTP 200
[Captures]
root_post_id: jsonpath "$[0].id"


# Create a chain of nested posts
POST http://127.0.0.1:3000/gutp/v1/post/create
[Form]
title: Reply Level 1
content: First level reply to root post.
author_id: {{cascade_user_id}}
subspace_id: {{cascade_subspace_id}}
parent_post_id: {{root_post_id}}
ext_link: https://example.com/level1
is_public: true

HTTP 200
[Captures]
level1_post_id: jsonpath "$[0].id"


POST http://127.0.0.1:3000/gutp/v1/post/create
[Form]
title: Reply Level 2
content: Second level reply.
author_id: {{cascade_user_id}}
subspace_id: {{cascade_subspace_id}}
parent_post_id: {{level1_post_id}}
ext_link: https://example.com/level2
is_public: true

HTTP 200
[Captures]
level2_post_id: jsonpath "$[0].id"


# Test deletion order for nested posts
DELETE http://127.0.0.1:3000/gutp/v1/post/delete
[Form]
id: {{level2_post_id}}

HTTP 200


DELETE http://127.0.0.1:3000/gutp/v1/post/delete
[Form]
id: {{level1_post_id}}

HTTP 200


DELETE http://127.0.0.1:3000/gutp/v1/post/delete
[Form]
id: {{root_post_id}}

HTTP 200


DELETE http://127.0.0.1:3000/gutp/v1/subspace/delete
[Form]
id: {{cascade_subspace_id}}

HTTP 200


DELETE http://127.0.0.1:3000/gutp/v1/user/delete
[Form]
id: {{cascade_user_id}}

HTTP 200


# ===== DATA INTEGRITY TESTS =====

# Verify empty results when querying with valid but non-associated IDs
GET http://127.0.0.1:3000/gutp/v1/post/list_by_author?author_id=non_existent_author

HTTP 200
[Asserts]
jsonpath "$" count == 0


GET http://127.0.0.1:3000/gutp/v1/comment/list_by_post?post_id=non_existent_post

HTTP 200
[Asserts]
jsonpath "$" count == 0


# Test pagination consistency
GET http://127.0.0.1:3000/gutp/v1/user/get_by_account?account=non_existent&page=0&pagesize=10

HTTP 200
[Asserts]
jsonpath "$" count == 0
