# Abstract

## TLDR

This is a Tor hidden service, similar to WikiLeaks, written in Rust, featuring
comprehensive documentation to support creation of a decentralized ecosystem for
the responsible disclosure of confidential information. Our vision is for
*resp0disc* instances to be established in multiple countries worldwide, each
working with the most trusted, transparent, and unbiased journalists native to
those countries.

## NOT TLDR

### 1) What's this ?

*resp0disc* stands for "**Responsible and Secure Disclosure of Classified
Information to Unbiased, Professional Journalists through a Localized Service.**
"

### 2) Why ?

This will get a bit philosophical, sorry; Simply, we feel as though we stand at
a crossroads, determining the future of global society;

On one hand, people have never enjoyed such unprecedented privileges as we do
today, especially in the Western world. Advances in healthcare, which would have
seemed unimaginable just two decades ago, have become a reality - prolonging the
life expectancy even further. In Western world, we've lived through an
extraordinary period of relative peace, marked by international cooperation,
though, regrettably, this does not extend to the situation in Ukraine.
Technology is advancing at an exponential rate across every field—from IT and
biotechnology to nanotechnology, space exploration, and medicine. We now have
access to tools and services that would have seemed like magic only few decades
ago.

On the other hand, while we enjoy unprecedented privileges, we also face the
highest number of existential threats that cast a shadow over our future.
Climate change is perhaps the most pressing of these, with rising temperatures,
extreme weather events, and environmental degradation threatening ecosystems,
economies, and livelihoods across the globe. At the same time, the rapid rise of
artificial intelligence presents its own profound challenges—ranging from job
displacement and ethical dilemmas to the potential for AI to surpass human
control, altering the very fabric of society or even destroying it as a whole.
Nuclear weapons proliferation is another dire concern. The specter of nuclear
conflict—whether through state tensions or accidental launches—remains a dark
reality, exacerbated by geopolitical instability and the breakdown of arms
control agreements. As a result, the threat of global destruction has not
diminished, but rather evolved in new and unsettling ways. Moreover, the rise of
social networks, which have revolutionized communication, also presents new
dangers. Platforms that were initially intended to connect people have become
fertile ground for manipulation, misinformation, and social fragmentation. The
Cambridge Analytica scandal serves as a stark example of how data harvesting and
targeted political ads can be weaponized to sway elections and shape public
opinion in ways that undermine democracy itself. The ability to influence and
divide populations through social media platforms, combined with the growing
power of artificial intelligence to manipulate information, poses a profound
challenge to social stability and individual autonomy. Another looming threat is
the demographic crisis in many Western nations, where declining birth rates and
aging populations are leading to a shrinking workforce and growing pressure on
social services. As the number of elderly increases and the working-age
population shrinks, the economic model that has supported these countries for
decades may no longer be sustainable. This demographic shift could result in a
scarcity of resources, rising healthcare costs, and strained pension systems,
leading to social and political instability. Compounding this issue is the rise
of populism. Populist movements, often driven by discontent with globalization,
economic inequality, and cultural change, have gained significant traction
across many parts of the world. These movements, which promise simple solutions
to complex problems, often capitalize on fear, nationalism, and distrust of
elites. The rhetoric of populism can deepen divisions within societies,
polarizing populations and undermining democratic institutions. Populist leaders
tend to focus on scapegoating minorities, exploiting economic and cultural
insecurities, and dismantling checks and balances. As a result, the rise of
populism poses a significant threat to democratic values, human rights, and
social cohesion, driving nations further apart at a time when global cooperation
is crucial to solving our shared crises. In this complex landscape, we are faced
with the paradox of unprecedented progress and grave risk, where the very tools
that offer us solutions may also be the ones that threaten our future. With
mounting challenges—climate change, artificial intelligence, nuclear weapons,
demographic decline, populism, and social fragmentation—we are at a crossroads.
The choices we make now will shape the world for generations to come.

In a world that is changing at such a rapid pace, we believe that access to
**accurate** and **unbiased** information is more crucial than ever. While some
parties may view some information as confidential, others—**whistleblowers,
dissidents**—may see it as vital for the public to know. The aim of this project
is to provide a responsible, fast, secure, and opensource solution to address
this challenge, including thorough documents describing how to deploy and run
the service, so any competent sysadmin could catch up and securely bring
instance of *resp0disc* to life.

### 3) Again - Why, We have WikiLeaks and such...

While WikiLeaks played a crucial role during the Iraq War and in several other
high-profile cases, we believe it has accumulated too much power, with just a
handful of individuals making decisions about what is released and which
preselected journalists receive the information. We believe that multiple
localized independent platforms like WikiLeaks would foster greater
transparency, leading to more important confidential information being made
available to the public. Our vision is for *resp0disc* to operate within
countries such as Romania, Slovakia, Bulgaria, and Ukraine, working with each
country's most trusted, transparent, and unbiased journalists.

# Prerequisites

* Linux based machine (server)
* Tor
* Rust
* Python 3.8+
* Docker
* Tor browser (preferred), or any other tor-proxied browser

# WIP (Stay tuned !)