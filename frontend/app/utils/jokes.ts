// Thanks to https://startit.bot/
// for this list of really awesome and funny jokes :3
const jokes: string[] = [
  `Dlaczego frontendowcom najłatwiej jest zamówić jedzenie z McDonalda?\nBo mają hamburger menu`,
  `Co robi component, który zmienia dom?\nRemount`,
  `Jaka jest ulubiona zupa Google Chrome?\nRAM-en`,
  `Co robi scram master po sprincie?\nMyje Jiry`,
  `Dlaczego nowa składnia java scriptu została doceniona przez środowisko łuczników?\nBo wprowadzono arrow function`,
  `Co frontendowiec mówi do konia?\nWiśta Vue`,
  `Dlaczego nauczyciele to kiepscy programiści?\nBo korzystają tylko z tablic`,
  `Dlaczego frontendowiec chodzi do opery?\nŻeby poogladać diwy`,
  `Jaki jest ulubiony król frontendowców?\nAugust III SASS`,
  `Jaki jest ulubiony pies frontendowca?\n<span>iel`,
  `Jak jest ulubiona część lotniska dla programisty?\nTerminal`,
  `Co frontendowcowi sterczy z nosa?\nBabel`,
  `Jak się nazywa błąd związany z wyświetlaniem elementu w przeglądarce Microsoftu?\nEdge case`,
  `Co mówi ksiądz na ślubie informatyka?\nPobieranie zakończone`,
  `Co bierze informatyk na turniej rycerski?\nKopię zaposową`,
  `Jaki jest ulubiony serial programistów?\nOpowieści skrypty`,
  `Jaka jest ulubiona kolęda frontendowca?\n<li></li><li></li> laj`,
  `Jaka jest ulubiona ryba programisty?\nPstring`,
  `Jakie jest ulubione zwierzę PM'ów?\nJirafa`,
  `Jaka jest ulubiona maść administratorów?\nSudocrem`,
  `W jakim kraju są najbardziej doświadczeni developerzy?\nW meksyku, bo tam każdy to senior`,
  `Jaka metoda HTTP jest najczęściej używana przed wielkanocą?\nPOST`,
  `Jaką chatę buduje frontendowiec?\nHT ML`,
  `Co najbardziej przeraża frontendowców?\nNawiedzony DOM`,
  `Ulubiona muzyka glonojadów?\nAlgorytmy`,
  `Po czym poznać że kod jest czysty?\nPo tym, że się myje`,
  `Jaki jest najgorzy rodzaj PH?\nPHP`,
  `Ulubiona bajka Node Developerów?\nNoddy`,
  `Dlaczego backedowcy to leniuchy?\nBo ciągle robią REST`,
  `Co łączy Rust i siłownię?\nTu i tu są silne typy`,
  `Gdzie programiści zbierają grzyby?\nW server lesie`,
  `W którym kraju programują najgorzej?\nWe Włoszech, bo ciągle piszą spaghetti kod`,
  `Zrobili nowy framework w Australii, jaki?\nKangular`,
  `Jak się nazywa dziedzina data science dla leniwych osób?\nData spokój`,
  `Czego fani Apple jedzą w święta najwięcej?\nMacowca`,
  `Co mówi właściciel sklepu z artylerią?\nU mnie działa`,
  `Powstał nowy sport, box z elementami jogi, nazywa się...\nflex-box`,
  `Wkurzona dziewczyna pyta programistę, co wybierasz. Wyjście do kina czy edytor kodu?\nA on mówi, IDE`,
  `Ulubiony framework jaszczurek?\nGadsby`,
  `Jaka jest najśmieszniejsza aplikacja na świecie?\nAdobe XD`,
  `Ulubiona bajka project managerów?\nGdzie jest Demo?`,
  `Co łączy programistów i policjantów?\nKomendy`,
  `Rodzice alergików zostali zapytani jak często dzieci maja katar\nNo i matka odpowiada, że córka ma sezonowo a syn chronicznie`,
  `Jak sie nazywa Ruby Developer na wczasach?\nRuby on rejs`,
  `Dlaczego PM'i znają się na napojach gazowanych?\nBo codzniennie robią calle`,
  `Jak się nazywa Developer, który ciągle robi sobię bekę?\nBeckendowiec`,
  `Na jakiej stacji tankują DevOpsi?\nNa Shellu`,
  `A ile trwało najkrótsze zlecenie dla Designera?\nAdobe`,
  `Jak się nazywa najpopularniejszy web master na świecie?\nPeter Parker`,
  `Dlaczego DevOpsi kupują nabiał u Weroniki?\nBo lubią serwery`,
  `Jak Czesi mówią na niewielkie elementy w HTMLu?\nMaledivy`,
  `Jak nazywa się syn JavaScripta?\nJSON`,
  `Jak wystraszyć React Developera?\nHookiem`,
  `Co się robi gdy chce się pochwalić React Developera?\nPropsuje się go`,
  `Ulubiony burger Steve'a Jobs'a?\nBig Mac`,
  `Czym Frontendowcy myją podłogę?\nAJAX'em`,
  `Skąd wiadomo że programiści mają dziwne fantazje ?\nBo trzymaj stringi w bibliotece`,
  `Jaki jest najlepszy sposób na naukę React'a?\nRozmowa z Native'm`,
  `Co słyszy frontwndowiec jak jedzie pociągniem?\n<td></td><td></td>`,
  `Ulubiony tag niemieckich developerów?\nGuten tag`,
  `Co łączy SQL'a i dupstep\nDROP`,
  `Skąd informatyk bierze wodę?\nZ E-kranu!`,
  `Dlaczego programiści mylą Boże Narodzenie z Halloween ?\nBo 25 Dec = 31 Oct`,
  'Z programowaniem jak z budową katedry, budujesz, budujesz a potem się modlisz (żeby wszystko działało)',
  `Co kupuje informatyk w aptece?\nWitaminę C++`,
  `Dlaczego drukarka nigdy nie wygrywa w wyścigach?\nBo zawsze jest tusz, tusz`,
  `Ulubiona część garderoby przez informatyków?\nStringi`,
  `Jak śni programista?\nNa JAVIE`,
  `A wisz jaki jest najlepszy edytor kodu?\nVim`,
  `Co mówi fasolka do groszku?\nStay strong`,
  `Co mówi pracownik supermarketu, gdy prosi o ostrożość?\nBe Carrefour`,
  `Co mówi Gerald, gdy chce o coś zapytać?\nHey, Ciri`,
  `Ulubiony taniec piekarzy?\nBread dance`,
  `Skąd pochodzi najstraszniejszy kod?\nSkrypty`,
  `Który z bohaterów miał najlepszą pamięć?\nRAMbo`,
  `Gdzie mieszka frontendowiec?\n W DOMu`,
]

export function getRandomJoke(): string {
  return jokes[Math.floor(Math.random() * jokes.length)] ?? ''
}
