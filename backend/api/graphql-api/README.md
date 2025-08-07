# GraphQL API

This crate exposes a GraphQL API for complex queries using `async-graphql`.

## Schema

```graphql
type User {
  id: ID!
  name: String!
  events: [Event!]!
}

type Event {
  id: ID!
  userId: ID!
  timestamp: DateTime!
  description: String
}

type Dashboard {
  id: ID!
  ownerId: ID!
  vibes: [Vibe!]!
}

type Vibe {
  id: ID!
  score: Int!
  message: String
}

type Query {
  user(id: ID!): User
  events(userId: ID!): [Event!]!
}

type Subscription {
  vibeUpdated(userId: ID!): Vibe
}
```

## Example Query

```graphql
query GetUserWithEvents($id: ID!) {
  user(id: $id) {
    id
    name
    events {
      id
      timestamp
      description
    }
  }
}
```

## Subscription Setup

Connect to the WebSocket endpoint and run:

```graphql
subscription OnVibeUpdated($userId: ID!) {
  vibeUpdated(userId: $userId) {
    score
    message
  }
}
```

The server pushes real-time vibe updates to connected clients.
