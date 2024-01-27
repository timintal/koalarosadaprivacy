use yew::prelude::*;

pub(crate) fn get_privacy_page(name:&str) -> Html {

    html! {
        <>
            <h1>{ format!("Privacy Policy for {name}")}</h1>
            <h3>{"Effective Date: 26th January, 2024"}</h3>
        <p>{format!("This Privacy Policy describes how your personal information is collected, used, and shared when you install or use the {name} game (the \"Game\") on any mobile device. This Game is developed by Ruslan Konysheu, an individual developer residing in Spain (\"we\", \"us\", \"our\").By installing and using the Game, you agree to the collection and use of your information in accordance with this Privacy Policy. If you do not agree with the terms of this policy, please do not install or use the Game.")}</p>
        <h3>{"1. Information We Collect"}</h3>
        <p>{"When you install and use the Game, we may collect certain information automatically, including but not limited to:"}</p>

<p>{"Device Information: We collect information about the device you use to access the Game, including the hardware model, operating system and version, unique device identifiers, and mobile network information."}</p>

<p>{"Location Information: We may use your IP address to estimate your general location."}</p>

<p>{"Usage Information: We collect information about your interaction with the Game, such as app launches, taps, video views, and other usage statistics."}</p>

<p>{"Diagnostic Information: We may collect information related to the performance of the Game and the Admob SDK, including crash logs, app launch time, hang rate, and energy usage."}</p>

<p>{"Identifiers: We collect device and account identifiers to provide our services, including advertising and in-app purchases.
"}</p>
        
        <h3>{"2. How We Use Your Information"}</h3>
        <p>{"We use the information we collect to:"}</p>
        <p>{format!("Operate and maintain the Game;
Provide customer support and respond to your queries;
Send you technical notices, updates, security alerts, and support and administrative messages;
Monitor and analyze trends, usage, and activities in connection with the Game;
Detect, investigate and prevent fraudulent transactions and other illegal activities and protect the rights and property of {name} and others;
Personalize and improve the Game and provide advertisements, content, or features that match user profiles or interests.
")}</p>
        <h3>{"3. Sharing of Information"}</h3>
        
        <p>{"We may share your information as follows or as otherwise described in this Privacy Policy:" }</p>
        <p>{format!("

With vendors, consultants, and other service providers who need access to such information to carry out work on our behalf;
In response to a request for information if we believe disclosure is in accordance with, or required by, any applicable law, regulation, or legal process;
If we believe your actions are inconsistent with our user agreements or policies, or to protect the rights, property, and safety of {name} or others;
In connection with, or during negotiations of, any merger, sale of company assets, financing, or acquisition of all or a portion of our business by another company;
Between and among Drop Craft and our current and future parents, affiliates, subsidiaries, and other companies under common control and ownership; and
With your consent or at your direction.
")}</p>
        <h3>{"4. Advertising and Analytics Services Provided by Others"}</h3>
        <p>{format!("We may allow others to provide analytics services and serve advertisements on our behalf across the web and in mobile applications. These entities may use cookies, web beacons, device identifiers, and other technologies to collect information about your use of the Game and other websites and applications, including your IP address, web browser, mobile network information, pages viewed, time spent on pages or in apps, links clicked, and conversion information. This information may be used by {name} and others to, among other things, analyze and track data, determine the popularity of certain content, deliver advertising and content targeted to your interests on our services and other websites, and better understand your online activity.")}</p>
        <h3>{"5. Your Rights"}</h3>
        <p>{"You have the right to access, correct, or delete your personal information held by us. You also have the right to object to or restrict certain processing of your personal information. If you wish to exercise any of these rights, please contact us at: "} <b>{"koalarosada@gmail.com"}</b></p>
        <h3>{"6. Data Retention"}</h3>
        <p>{"We retain your personal information for as long as necessary to provide you with the Game and for legitimate and essential business purposes, such as maintaining the performance of the Game, making data-driven business decisions, complying with our legal obligations, and resolving disputes."}</p>
        <h3>{"7. Transfer of Information to Other Countries"}</h3>
        <p>{"Information collected through the Game may be stored and processed in any country where we or our service providers have facilities. By using the Game, you consent to the transfer of information to countries outside of your country of residence, which may have data protection rules that are different from those of your country."}</p>
        <h3>{"8. Children's Privacy"}</h3>
        <p>{"The Game is not intended for individuals under the age of 13. We do not knowingly collect personal information from children under 13. If we become aware that we have inadvertently received personal information from a child under the age of 13, we will delete such information from our records."}</p>
        <h3>{"9. Changes to this Privacy Policy"}</h3>
        <p>{"We may update this Privacy Policy from time to time in order to reflect, for example, changes to our practices or for other operational, legal, or regulatory reasons. We will notify you of any changes by posting the new Privacy Policy on this page."}</p>
        <h3>{"10. Contact Us"}</h3>
        <p>{"If you have any questions about this Privacy Policy, please contact us at: "} <b>{"koalarosada@gmail.com"}</b></p>
        </>
    }
}