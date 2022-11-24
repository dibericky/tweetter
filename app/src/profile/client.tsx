import { ApolloClient, InMemoryCache, ApolloProvider, gql } from '@apollo/client';
import React, { useState } from 'react';
import { Params } from 'react-router-dom';

const client = new ApolloClient({
    uri: 'http://localhost:8080/graphql/',
    cache: new InMemoryCache(),
});

const query = `
query($userId: String!) {
    user(userId:$userId) {
         id,
      numTweets,
      nickname,
      follower,
      following,
      createdAt
    }
    userTweets(authorId:$userId) {
      id,
      author {
        id,
        nickname
      },
      message,
      createdAt,
      updatedAt
    }
  }
`

export type Tweet = {
    id: String,
    author: Author,
    message: String,
    updatedAt: string,
}

type Author = { id: String, nickname: String }

export type User = {
    id: String,
    nickname: String,
    numTweets: Number,
    following: Number,
    follower: Number,
    createdAt: string
}

export type Response = {
    user: User
    userTweets: Tweet[]
}

export async function loadUserData({params} : {params: Params<string>}) : Promise<Response>{
    let userId = params.userId;
    const response = await client.query({query: gql(query), variables: {userId}});
    return response.data
}
