import { ID } from "./common";

export interface Comment {
  body: string;
  id: ID;
  insertedAt: number;
  remoteIp: string;
  signature: string;
  updatedAt: string;
  url: string;
  website: string;
}

export interface CommentInput {
  body: string;
  signature: string;
  url: string;
  website: string;
  email: string;
  iAmARobot: boolean;
}
