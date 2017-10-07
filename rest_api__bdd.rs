//! 1. Simple -- R.Hickey, ***simple - NJ-style(R.Gabriel)
//! 2. mut_aliasing -- several links, AT_LEAST one of them is MUTABLE
//!    BTW: bunch_of_closures_with_mut_refs is like obj which alias through its params, which is
//!     mut_aliased refs on some another obj, or shared data
//!         that_is:  CLOSURE_CAPUTRING == OBJ.MUR_ALIASED_REFS


##### TEMPORARY
// Quotation:
// | When I say “less code”, I don’t mean less lines of code: I mean code that does less. A fast, 
// | convenient solution or a clever, terse solution are rarely the best ways to solve a problem. 
// | Both reduce immediate costs—time or lines of code—in exchange for greater maintenance costs 
// | down the road. 
// ..
// KISS, W-B, simpler realization == less bugs

## 2
// SRP --> Data-cohesion --> minimize scope && incapsulate

## 3
[api-first-design](http://www.programmableweb.com/news/how-to-design-great-apis-api-first-design-and-raml/how-to/2015/07/10)  
// ^----  Here is described about importance API first desing and it's similar
// to main.jl:@16.1, here API-design importance is like importance to write
// functional(or maybe more correctly "acceptance") tests to you programm-api,
// and in @16.1 I have wrote "initially it's important just to describe what we
// want in "strings what we pass to describe and spec generally"" and it's similar
// to describing REST-API in RAML  
|  
v  

## 4
// https://habrahabr.ru/post/249183/#comment_8256363  
// Eto primer togo chto chuvak ne v'ehal pohodu(a mozh 9 poka ne v'ehal :lol:)
// Kak ktoto citiroval M.Fauler: "Servise eto vse chto mozhno vynesti v otdel'nyi
// component. Esli chto to nel'z9 razbit' na servisy izza sv9zannosti, to znachit
// eto i est' odin servis", potomu esli u teb9 problemi s tem chto ty vidiw v
// servise chto on rewaet neskol'ko funkcii, i ego mozhno razbit' na neskol'ko
// microservisov chto rewaut kazhduu iz etih problem(funkcyi), no tebe eto trudno
// sdelat' izza suw'estvennoi sv9zannosti ego DB, to znachit scoree vsego emu luchwe
// ostat's9 edinstennym servisom. {
//  also: v "podcaste o microservisa SoftEngDailyRadio" tip govoril chto nuzhno ogranichit'
//  "vyzovy"(rpc/rest) mezhdu servisami (tobiw ne tak wob hodit' v drugoi servise
//  na kazhdyi chih)  ... takwo eto takzhe kriterii .. esli u vaz "post9nnye sv9zi"
//  s drugim sericom .. to vozhmozhno eto dolzhen byt' odin service
// }
// Takzhe, upominalos', chto nekotorye uparyvauts9 do togo wob razbivat' servise
// na programmi po 100 strochek, NO smysl ne v tom chto by razbit' vse na micro
// chasti, a chto by ne bylo monolita kotoryi trudno podderzhivat', potomu
// neob9zatel'no uparyvat's9
|4.1  // ... to be continued ... 
//
// from: http://www.ben-morris.com/how-big-is-a-microservice/   <<  GODNYI POST, S GODNYMI SSYLKAMI
// ..
//  -=  "In service development, autonomy is much more important than size"  =-
//  ..
// "You have to start somewhere and the more your build the more obvious it becomes where
//  the boundaries between your services should be. Whenever you encounter something that looks 
//  like it should be separate then you can spin it out as a new service."


## 5 // (perevedi na eng)  
// offtop: "Pravila lubogo frameworka: osnovy/principy dolzhny byt' kraeugol'ny i
// nepokolebimy, no odnovremenno on i ne dolzhen zakamenit's9 na konckretike"


## 6 >> moved ::tdd_bdd.rs

## 7
//!  So .. conclusion for now: every data you are using in the function should
//!  be mutated only in this function, and every data you mutate in the function
//!  should not be read/write in other functions bellow-in-stack(during execution your
//!  function - that is while you borrow it)..[p.s. EXCEPT IF YOU PASS THIS PERMISSION EXPLICITLY]
//!  and above(if you cally)
//!      ^-- NO MUT_ALIASING..
//!         .. since we need "prepare state" before calling "some_fn", but IT'S NOT OBVIOUS
//              (although in Actor#move it's obvious and on_purpose)
//
class Actor {
    moveTo(x, y) {
        this.x = x; this.y = y;
        // only AFTER we mutate `this`:Actor that makes sense to `draw` scene
        this.scene.draw();
    }
}
// ... in JS, I should READ the doc (IN BEST CASE) or src(IN WORST CASE) to KNOW that I should do
// `scene.draw` after "assignments"..
//      (or PREPARE "SPECIAL_STATE" for `draw`, to achieve desired side-eff)
//  .., and here it's "LIKE" obvious, it's "LIKE" intentionally, ..
// .. BUT in many cases it's "NOT OBVIOUS" and people don't do it "INTENTIONALLY", but ACCIDENTALLY
// that causes a BUGs.
//      ..
//      ^-- BUT according to W-B .. it's can be ***simpler, ("i do it on_purpose") espesially
//      on the stage of "prototyping" or "early designing" ... SO ..
//                  ... THE HACKER WAY IS DISCOVER THAT TRADEOF

## 9 >> moved ::tdd_bdd.rs

## 10 >> moved ::tdd_bdd.rs

## 11 // (inspired by youtube:"How to design a good api and why it matters" 2007)
//
// Someone can say: "We need add new data structures and RPCs with Version2 attributes"
// You here it as:  "We need a new data format that accomodates evolution of attributes"
// So .. if you need new thing(feature) add to your programm, you need to think not about this
// partikular thing, but what we add to your programm generally (!about "CONCEPT" <-- see below).
// And .. of course if you try to write this general API for this new CONCEPT[thing](here "data
// format") it should be USE_CASE oriented to be not (leak or/and fat) abstraction
//
//      about solving what should be in your API
///"WHEN IN DOUBT, LEAVE IT OUT",
// usually you can extend your API later, but it's too hard too cut it down
//
// a little bit abstract claim(I'm not sure I fully understand it)
///"when we talk about the size of API - the number of conceptions is more import than
/// number of methods,classes,params,etc"
// .. SO how many things(CONCEPTs) I should learn when I familiarize with API .. AND .. 
// .. `power-to-weight RATIO`(reusability) should be good .. AND .. the biggest `ratio` is in ..
// .. tada .. Monad, Iterators .. so reusability of API matters --
// -- "you should be able doing a lot without learning a lot"
// one trait/typeclass/interface - one concept,  API-of-service should usually have not very
// much concepts.  (also AFAIU `concept` is in 1-to-1 relation with ACDE)
// 
// and of course ..
/// Implementation should not has impact on API, don't let implementation "leak" into API
// frMe, again you have the place where we want some `functionality`(API), we describe what
// functionality(concept) we need in this place(USE_CASE oriented) and only after that we care
// what concrete structs/classes can be suited to implement this API(again: ACDE is not about
// modeling the world and describing phenomena, it's about abstraction).
//
/// Any API is like a little langueage .. so make good names for methods and concepts
//
/// When your API has bad documentation it's cause user to read the code of implementations, and
/// prompts him to write implementation-bounded code, instead of just rely on abstraction(and this
/// tend that if you break you implementation it's start(although it should'nt) impact on user, as
/// exmp: in BikeShedding-podcast was mentioned that users of Rails rely on Rails privat API )
// p.s.: that what I'm as user often do, I feel uncomfortable if I just use API but don't know
// how implementation work :lol: (but I hope it's because of my desire to research my tools :lol:)
// also
/// Document all side-effects
// Hah, it's also mention about documenting ownership, and ability mutate(again: side-eff) params
// and result of meth ... that what I'm caring about now, and the initial reason why I start listen 
// this lections (recomended by Er.Normand) .. so tada .. In RUST it's INTEGRATED in types, win!

## 16
// 1. O univarsal'nosti monad: from theoretic perspectiv it's cool. For exmp remember my thoughts
//    that monads let's to apply one func to diff contexts(list, or IO) so ..
someFun a = return (a + a) // << Haskl ..
//  ...  let us to use the same fn as in IO monad, as in List monad ,  BUT ,on practice it's TOO
//  RARE, for exmp consider `Promise` monad in JS,   IT'S VERY UNLIKELY that the same fn can be
//  used as in Promis as in `arr.map(fn)`  SO,    IN RUST we don't bother about HKT, since on
//  practice it's not necessary (it's only necessary in PURE-func langs[formalism fuck])   ...
//  .. MOREOVER  if we have such situation as with `someFun` we can do like:
struct IO {
    fn handle_io(p: Val) -> IO { IO::return(pure_func(p) }
}               // both types incapsulate side-eff(like Promise in JS)
struct List {
    fn handle_list(p: Val) -> List<T: Add> { List::return(pure_func(p)) }
}
fn pure_func(p: i32) -> i32 { p + p }
// .. SO ... here we do similarly how in JS I decide not make `funcs what generate new fns with
// single pattern, and parametrize them by specific fns` BUT write by hand every fn, and EXTRACT
// similar parts as handlers(here it's `pure_func`)
// p.s In JS I did --v,   BUT NOW I define usually every fn separately and use helpers instead of
function define_fn(obj, name, f) {                           // `some pattern ***`
    obj[name] = function () {
        some pattern first part ...; let arg = .... ;
        f(arg); some pattern second part.. ;
    }
} 
// ..
// .. MOREOVER, since in NODE we don't use SYNC io(in 99% cases(I don't take into account `debug 
// logging`, and `read configs`), it's BAD STYLE), and if we use Immutable.js, and Promises, ..
// .. THEN ... TADA  .. JS is PURE LANGUAGE, since we have pure funcs(non
// mutability-side-eff since Immutable.js), we have non-pure-fns(which is BINDED to each other by
// Promise -- keywork `async` describe IO-unpure)   .. WIN
//
// Moreover we can even define something like Transformer for IO in Rust
struct IO<T> { }
impl<T> IO<List<T>> { ... }
//
// Also .. about JS GENERATORS, maybe its profitability is that they incapsulate mutability.
// So, in cases when our async fn(cb) is rely on some glob(at least relativly them)
// variable(which it capture in closure(it is closure/fn/cb))  WE CAN USE GENEREATOR(or actualy
// it's iterator for that moment) TO INCAPSULATE THIS MUTABLE STATE (see practical cases(not so
// well suited, but for start it's good) in ponyfoo[async io with promises and generators])


## 18 >> moved tdd_bdd.rs

## 19
// How I understand DIP(SOLID#5).  It's mistake to write structure, and try too invent interface
// for it, OR maybe its SRP, :lol: since my "Graph-interface mistake" is also DIP breaking, since
// implementations drive me to invent interface for them, BUT "not implementation should drive
// abstraction, but abstraction should drive implementation".
// .. SO, interface should be "thin" and "USE-CASE" oriented, and don't be driven by any
// realization(like graphs makes me create Graph trait -- NAHU9?)   BUT ONLY INTENTION which we
// impose on it interface( Our needs drive the forming of interface ..) that is the fact that we
// need some "Store" drive interface, instead of the fact that we have several DBs and we want
// invent some interface for them  .... if we have many DBs and want to invent API for them ==>
// realizations "nav9zyvaut nam" this API , BUT IT'S WRONG,   our NEEDs should "nav9zyvat' nam" API
///     DON'T MESS it with W-B -- where we ON-PURPOSE make our API "worse", to achieve
///     ***simplicity in implementation .. but not as DIP-break .. where we without any
///     benefits of ***simplicity write API in "wrong"(implementation-"driven") way
///
// cool exmp thinking "OOP is not about real-world object" and how it related to DIP:
// ..
// f64 and i32, from common person(especially math) have some relation -- they both NUMBERs,
// .. BUT.. from programming perspective they have NO RELATION AT ALL!!  Thay just 2 diff unrelated
// definitions how struct some bunch of bites, they arranged differently, operations on them
// implement differently,   and only reason why we can treat them(!! from programming
// persepective !!) as something with similar concepts -- it's that they share implementation of
// some traits, that's it has some functionality in common  ..
// ... So don't try drive your abstact thinking about some type by orienting on your "perception"
// of this type, TYPE DON'T DRIVE INTERFACE, the fact that your imagin that Float and Int is
// something with common (math) concepts SHOUD NOT DRIVE YOU TREAT THEM AS SUBCLASSES of PARTICULAR
// SUPERCLASS,  from programming perspective they have similar concepts only since they
// implement similar traits -- which driven by "use-case orienting".
## 19.1
// Also: I read about Hexagonal-arch ... so there also similar exmp, our business logic, interact
// with "outside-world" through adapter, which represent object with our USE-CASE oriented (and
// MOST GENERAL AS POSSIBLE (i think so now)) interface

## 20  redirect to ## 18

## 21 >> moved to ::tdd_bdd.rs


## 22
// Quotation:
// | But Grunt’s failure wasn’t configuration. It was a lack of opinion.
// | Configuration isn’t evil. It’s a powerful time-saver when the assumptions are right.
// ..
// For general configuration "gulp" is cool, since JS is much more `agile` lang to express any
//      possible case,
// And  "opinionated" configuration with "config-file format" is also cool, since it more concise ..
// ...  BUT .. don't try to do general "configuration" in config-file format, here
// PROGRAMMING-lang(dynamic, or with meta-programming facilities) is much better


## 23
// http://www.javaworld.com/article/2077873/core-java/core-java-creating-dsls-in-java-part-2-fluency-and-context.html?page=2
// .. 
// this web-site called "javaworld", but the author is heavy-"rubyist-world" person
// ..
// So: [GIST] yeah, author is describing (e)DSLs .. and that why he compare programming
// langs(here eDSLs) with natural-langs ... BUT .. GENERALLY .. he try to think about programming
// as human-description  ... SO..
// AGAIN: programming is just FORMAL LANG .. that all .. it's good experiment try to think about
// programm code as a human-lang .. BUT IT'S NOT MAIN WAY you think about software .. it's just
// alternative (in the far dark corner)
// SO .. exmp:
// 
//
// Also he praise this as "fluent" code --v
expect(alarm.raise()).andReturn(true);
expect(alarm.raise()).andThrow(new InvalidStateException());
// man, don't be attracted by "easiness"(simple vs ease) .. MOST OF ALL - SIMPLICITY ..
// again:   code with less nums of layers of abstraction and "nice" interfaces is better
//          then super-"ergonomic" code with many layers of abstraction
//          "cena/kachestvo"  ~~ "ergonomicity/layers_of_abstr"
// not fool yourself, don't be attracted by "human-lang like `code`-narration" -- IT'S REDUNDANT
// COMPLEXITY of THINKING about code .. more SIMPLE(Hickey) but not "fluent"(FORGET about that 
// invented shit) code is much better  -- don't try to "her9ch' eDSL i piwy imi vsu
// programmu" - aka Ruby-n-Rails style...  if this eDSL has big popularity you can somehow cope
// with it, since at least this eDSLs is "standard", and web-dev is the case where eDSL sometimes
// really matter .. and Ruby-n-Rails is used in situation "her9k-her9k i v prodakwen, a potom
// perepiwem eto na Scale" aka "worse-is-better".
//              eDSL -- ONLY IN SOME CASES WHERE IT'S REALLY HAS MATTER.
// ..
// Also, exmp
cart.with {
  add "Milk"
  add "Juice"
  println "My cart has $size items."
}
// in JS it's just
cart.with(() => {           function cart::with(fn) { fn.call(ctx) ....  }
    this.add("Milk");       // In JS it's easier just past the PLAIN-OBJECT
    ...                     cart.with({ .... })  // and don't bother with "HUMAN-like lang"..
});                                             // .. STUPID (invented) ASSOCIATIONs
                            // or
                            cart.with(ctx
                                .add("Milk")        // <<- It's just fucking BUILDER, NAHUI eDSL
                                .add("Juice"))
// So.. : Ruby just HIDE `this`, and because of that (syntactic sugar) .. people like/try to
// invents some "conversation-like context", and other traits of "human-lang" ..
//          .. IT'S JUST FUCKING eDSL .. NAHUI "HUMAN-Lang like traits"
//
//! .. Metaprogramming, macros, high-order-fns(for "ergonomicity") .. all stuff what make code
//! more ergonomic should be used ("tochehno") only for "skrait' ugly", that is remove redundant,
//! boring code - JUST "DRY"
// ..
//! .. "fluent", "context (of convers)", "OO - model code like RealWorld-obj relation",
//! "code like human-lang" --- DONT CLOG YOUR HEAD  ..
//    (stuff ( how "seacrh of HUMANITY-traits in code" impact):
//      - "it's enogh just "a nod" for me-with-friend to understand each other, since we know
//        the "CONTEXT"  )
//      - "it's enough just `add` since programmer understand context (but "IMPLICIT CONTEXT" ALSO 
//            INTRODUSE COMPLEXITY -- famous "Rails magic" -- again: eDSL - TOCHECHNO ..
//!         .. all narrows down to "CHOOSE RIGHT COMPROMISE" - where it make sense to use eDSL)"
//        ..
//! good-coding is all about make code - reusable, simple, and localize the problems(
//!   - like in "helpers what `eliminate coupling`", or
//!   - `locality` in R.Gabirel quotation - SEE: main.js:@26, or
//!   - "test" should help localize "search" of "future-bugs", doen't "100%covering to make my
//!     code super-relieable and without bugs(don't fool yourself)").
|23.1
// also: similarly to "native lang" ..
//      OOP(subtyping(private, protected, interfaces_ws_abst_classes, etc),
//          private/protected,
//          which class should own the helper,
//          which class should own the method[if two ebject kind of equally participate in method])
//      TDD(write test before code,
//          how to refactor my tests,
//          how make code-test-refacotr loop more fast,
//              [IF YOU USE TEST_CODE_REF-LOOP .. just run test only about changed peace of code
//                  .. as DHH said..
//                if your so unconfident, and scare that any little change breaks YOUR ENTIRE
//                CODEBASE ... the problem in your code, NOT in SPEED OF TEST_CODE_REFACT-LOOP
///               TESTs SHOULD BE FAST ENOUGH]
//          GOOD analogy from comment in E.Bugaenko blog about:
//              ALPINIST .. with TDD he stop move most "Simple" way{ like he do it without
//              "VEREVKI"}, and sine "verevka" "like" "strahuet" he , he becomes more confident
//              and start "ispoln9t' huinu"(like [over]"refactoring"/[over]"designing"), that is
//              he is distracted from "GIST" aka "efficient path" aka "what we want"
//              .... where as its "colleague" because tons of tests doen't do it "confident"(na
//              pustom meste) .. MAKE CODE ***SIMPLER , "ne pozvol9et sebe finty_uwami"
//              ..
//              Also @DHH mention how TDD makes people do stupid things (like J.Weirich with hes
//              "decoupling from rails")
//              quoute from TEST_DAMAGED DESIGN:
//                  """"
//      Such damage is defined as changes to your code that either facilitates 1. easier
//      test-first, 2. speedy tests, or 3. unit tests, but does so by harming the clarity
//      of the code through 
//                  """"
//          )
//      ..
/// ALL this concepts create NOISE .. which bring to many new concepts which supposed to help you
/// think about code .. but actually you think IN THIS NEW CATEGORIES .. which "GARBLE" the GIST


## 24 >> moved ::tdd_bdd.rs

## 25
// AGAIN: about "TEMPORAL COUPLING"
// ..
// It's basicly mean that imperative code has implicit( line-by-line ) "comp-order" during
// which we change the state,
// functional programming has NO IMPLICIT "comp-order", so that why in Yegor256 expm about
// "post-get-reguest" he complain about "temporal coupling"

## 26  >> moved into "idempotent.rs"

## 27
// You need insert "assert" (only) in places where "if we here - it's definetly bug (or at least
// 90%)"  ..  that's why IF we would handle it as "error" we do STUPID thing, since we
// "popustili bagu" for sake the system work further (we need it not just work, but work correctly)

## 28
// I often hear phrase(Crichton,Turon,Lerche) "iterate over design"  ..
//  .. So.. maybe it yet another subtle "mudrost'" which I now realize by "kalibrovka", and
// now I can make step back .. and realize that ..
// ..
// ... instead of understand "how design", I should learn how orginize development and start
// project and develop it in such way to quickly-n-wisely "iterate over design"   ..
// ... that is like "HW" .. not learn how "super-design-everything", but learn
// technique of "fast moving" in "prototyping" and trying(and "trying-to-undestend") correct
// design.
// ... ala "learn how to do bruteforce of "understanding/ideas" in most efficient way" :lol:


## 29
/// read: https://msdn.microsoft.com/en-us/library/jj591573.aspx .. CQRS and domain-driven design
// 
// the point here is that q`bounded context defines the context for a model`  .. earlier I think
// about how to "structurally abstract over" "(micro)service/dll" .. BUT .. (by DDD) when we build
// a system for some problem .. we divide this "problem" on "models" with "context" ... WE DON'T
// NEED ABSTRACT over this "models" since they in single exmpl ... abstractions applied INSIDE
// a "model" on more deep level .. that is "inside" (micro)service/dll

## 30
//  see: ::tdd_bdd.rs:##21
//  ... i o "oop modulieruet mir" - eto bred
//  ..
//  So .. "oop modulieruet mir" is fail << Yes, that's true .. BUT .. in context of DDD
//  it can be "justified" since  the gist in DDD  (AFAIU) is reduce the gap between
//  "dev" and "domain-expert"
//  ... SO..
//      .. for "dev" it's OBVIOUSLY NOT TRUE .. but for make "COMMON-LANG" between them
//  "dev" have to  "express" design in terms of "domain-model"
//  ..
//  And about ::tdd_bdd.rs##21 .. here the same thing .. Cucumber created to communicate
//  with stackholder .. that's why invented this STUPID roles.
//
//  I still fucking angry about this shit, but at least it can be somehow "justified".
//
//  (nevertheless, even with "justifications" it's seems STUPID, 
///     DHH: send me a "magic-dust" from the "world" where stackholders write "Cucumber-features")


## 31
//  https://habrahabr.ru/post/151169/
//  
//  ^-- repost this shit a lot time ago (~2014), when I was "uporotyi" Ruby-shit-OOP-shit
//  ..
//  MOST STUPID QOUTE from this article:
//      Но у нас нету каких-либо хороших способов описать взаимодействие объектов между собой
//      в рантайме для выполнения сценария использования. 
//  ..
//  Why stupid? Because LOCALITY(R.Gabrial) breaking, you should write code in the way you 
//  DONT NEED KEEP IN MIND all relations between many types/classes/objects
//      (autor just try describe Context/Transaction/Command to group some set of operations on
//      objects)
//  ..
//  But generally man in comments describe all this article in just few worlds:
//      https://habrahabr.ru/post/151169/#comment_5126646
//      ..
//      actualy: very similar to my Game class in Hero&Monster exmp in Ruby-OO-guide
class Game {
    fn constructor(@hero, @monster) { }
    // ... tut mozhno dal'we vhu9rit' i Builder-pattern dl9 construckcii scenari9, i e'we raznoi
    // huini po nuzhdam-i-zhelani9m
    //   ..NO.. svodits9 vse k odnomu ... Novyi tip dannyh combiniruuet drugie, i predstavl9et
    // soboi kakuuto novuu semanticheskuu-edinicu so svoimi "methodami", v dannom sluchae eto
    // "Context"
}

## 32
///  about: https://signalvnoise.com/archives2/it_just_doesnt_matter.php
// 
// and after that read comment of **Nathan Rutman**
//
// So I had remembered about "X system live for decades because of it's incredible flexibility,
// since philosophy: `Instrument not politics`"
//  ... BUT ....
// Doesn't easier to build more simple and opinionated (and appropriate) software which serve
// (exmp) 5 years (LESS feautres, and abilities to add-features/configure), and after time, if
// it doesn't fit, just build NEW software which is MORE APPROPRIATE to new conditions insteak
// just try "perelepit" old software ....
// ... someone can say .. but this new software will be not tested/proven/buggy-at-first-time
// BUT ... the gist is SIMPLICITY (not only interface/features but IMPLEMENTATION also) ... soft
// should be simple in implementation and not necessary super-flexible, but of course WITH THE GAP
// TO EXTENSION in SOME degree.
//
// GENERALLY the rule: if agility-in--(interface/features) makes implementation significantly
// complex, it's better eliminate this extesnibility/agility---for---features/extensions


## 33   // about Feathers
//
// Several things that makes difficult to understand what's going on in "Feathers"
//          (they all actually prettie similar)
// 1. It was not obvious how "feathers" register it's services.
//      .. and than I found 'feathers-rest'
// 2. But after that it was not obvious why "feathers" doesn't handle callback, and after
//      that I (accidentally) rearead `mixins/promise.js` and found answer ....
//      ..
//      That togather make "feathers" a little bit magical(over hackered :lol:)
//

## 34
// - sass ... do one thing and do it well +++ flexibility, since they don't bother about syntax
// - loopback ... your business to deliver a way(transport) for API (basically rpc) and
// rest-or-NOTrest we will build on top of that should not be hardcoded.
