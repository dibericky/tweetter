import { ApolloClient, InMemoryCache, ApolloProvider, gql } from '@apollo/client';
import React, { useState } from 'react';

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
      following,
      follower
    }
    userTweets(authorId:$userId) {
      id,
      authorId,
      message,
      createdAt,
      updatedAt
    }
  }
`

export enum QueryState {
    Loading = "loading",
    Solved = "solved"
}

type User = {
    id: String,
    numTweets: Number,
    nickname: String,
    following: Number,
    follower: Number
}

type Response = {
    user: User
}

export type UserData = {status: QueryState.Loading, payload: null} | {status: QueryState.Solved, payload: User } 


export function useGetUser() {
    const [userData, setUserData] = useState<UserData>({status: QueryState.Loading, payload: null})
    React.useEffect(() => {
        client
            .query({
                query: gql(query),
                variables: {userId: "7154c2f0-f265-4d1f-a54d-1fb2d628abe4"}
            })
            .then((result) => {
                let data : Response = result.data;
                setUserData({status: QueryState.Solved, payload: data.user})
            });
    }, []);

   return userData
}
