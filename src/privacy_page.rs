use yew::prelude::*;

pub(crate) fn get_privacy_page(name:&str) -> Html {

    html! {
        <>
            <h1>{ format!("Privacy Policy for {name}")}</h1>
            <h3>{"Effective Date: 26th January, 2024"}</h3>
        <p>{format!("This Privacy Policy describes how your personal information is collected, used, and shared when you install or use the {name} game (the \"Game\") on any mobile device. This Game is developed by Ruslan Konysheu, an individual developer residing in Spain (\"we\", \"us\", \"our\").By installing and using the Game, you agree to the collection and use of your information in accordance with this Privacy Policy. If you do not agree with the terms of this policy, please do not install or use the Game.")}</p>
        <h2>{"1. Information We Collect"}</h2>
        <p>{"When you install and use the Game, we may collect certain information automatically, including but not limited to:

Device Information: We collect information about the device you use to access the Game, including the hardware model, operating system and version, unique device identifiers, and mobile network information.

Location Information: We may use your IP address to estimate your general location.

Usage Information: We collect information about your interaction with the Game, such as app launches, taps, video views, and other usage statistics.

Diagnostic Information: We may collect information related to the performance of the Game and the Admob SDK, including crash logs, app launch time, hang rate, and energy usage.

Identifiers: We collect device and account identifiers to provide our services, including advertising and in-app purchases.
"}</p>
        
        <h2>{"2. How We Use Your Information"}</h2>
        <p>{"We use the information we collect to:

Operate and maintain the Game;
Provide customer support and respond to your queries;
Send you technical notices, updates, security alerts, and support and administrative messages;
Monitor and analyze trends, usage, and activities in connection with the Game;
Detect, investigate and prevent fraudulent transactions and other illegal activities and protect the rights and property of Drop Craft and others;
Personalize and improve the Game and provide advertisements, content, or features that match user profiles or interests.
"}</p>
        </>
    }
}