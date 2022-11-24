import { useRouteError } from "react-router-dom";

type Error = { statusText?: String, message: String }

export default function NotFound() {
  const error : Error = useRouteError() as any as Error;
  console.error(error);

  return (
    <div id="error-page">
      <h1>Oops!</h1>
      <p>Sorry, an unexpected error has occurred.</p>
      <p>
        <i>{error.statusText || error.message}</i>
      </p>
    </div>
  );
}