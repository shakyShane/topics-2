import '../styles/globals.css'
import type { AppProps } from 'next/app'
import {GetServerSideProps, GetStaticProps} from "next";

function MyApp({ Component, pageProps }: AppProps) {
  console.log(pageProps);
  return <Component {...pageProps} />
}

MyApp.getInitialProps = async ({ Component, ctx }: AppContext) => {
  console.log('here', ctx.router);
  return {
    props: {name: "shane"}
  }
}

export default MyApp
