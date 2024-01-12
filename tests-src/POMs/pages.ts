import {Page} from "@playwright/test";


export const basePageFactory = (page: Page) => {
  const locators = {
    headshot: page.getByTestId('Headshot'),
    navTree: page.getByTestId('NavTree')
  }
  const pages = [
    {name: 'Home', url: '/'},
    {name: 'My Work', url: '/my_work.html'},
    {name: 'Bookshelf', url: '/bookshelf.html'},
    {name: "Testimonials", url: "/testimonials.html"},
  ] satisfies Array<{ name: string, url: string }>;
  type PageName = typeof pages[number]['name'];
  
  return {
    pages,
    goTo: async (pageName: PageName) => {
      const targetPage = pages.find(
        p => p.name === pageName
      );
      if (!targetPage) throw new Error(`No page found with name ${pageName}`);
      await page.goto(targetPage.url);
    },
    getNavLink: (name: string) => locators.navTree.locator('a', {hasText: name})
  }
}
export type BasePage = ReturnType<typeof basePageFactory>;