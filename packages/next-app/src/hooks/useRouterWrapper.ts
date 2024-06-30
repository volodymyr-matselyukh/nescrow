import usePageNavigationStore from "@/store/pageNavigationStore";
import { useRouter } from "next/navigation"

export const useRouterWrapper = () => {
  const { setIsNavigating } = usePageNavigationStore();
  const router = useRouter();

  const routerPush = (url: string) => {
    setIsNavigating(true);
    router.push(url);
  }

  return { routerPush };
}