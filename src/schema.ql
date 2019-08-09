type Post {
    title: String
    views: Int
    user: User
    tags: [Tag]
}

type User {
    name: String
}

type Tag {
    name: String
}