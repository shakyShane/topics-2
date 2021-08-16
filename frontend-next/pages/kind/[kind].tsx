import {useRouter} from "next/router";
import {GetServerSideProps} from "next";
import {KINDS} from "../../src/types";

export function KindPage() {
  // const {} = useRouter();
  return <p>Here</p>
}

export const getServerSideProps: GetServerSideProps = async (ctx) => {
  const { params } = ctx;
  const kind = (params?.kind) || "" as string;
  const isValid = KINDS.includes(kind);
  if (!isValid) {
    return {
      notFound: true
    }
  }
  return {
    props: {}
  }
}

export default KindPage;