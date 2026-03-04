<script lang="ts">
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import { verify } from "$lib";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import { onMount } from "svelte";

    const cards = [
        {
            title: "Woodstock",
            description:
                "Our platform helps researchers collaborate more effectively by turning uploaded research files into a shared, searchable knowledge system. Instead of manually reviewing documents, users can ask questions and receive clear answers about ongoing projects, progress updates, and related work.                 By understanding the meaning behind the content—not just keywords—the system makes it easy to track developments, discover collaboration opportunities, and stay informed about fellow researchers’ work. The goal is to make research more connected, accessible, and efficient.",
            image: "./platform.png",
            link: "/login",
        },
        {
            title: "InnoRenew - IoT Platform",
            description:
                "Our IoT platform collects and aggregates data from connected devices in real time, transforming it into clear, actionable insights. Sensor data is visualized directly on interactive floorplans, dashboards, and charts, making it easy to monitor environments, track performance, and identify issues at a glance. The platform centralizes complex device data into a simple, intuitive interface for smarter decision-making.",
            image: "./iot.jpg",
            link: "https://iaq.innorenew.eu/grafana/public-dashboards/535d826acc744109b4d64a68654ee262?orgId=1&refresh=10s",
        },
        {
            title: "TIMBERHAUS Focus: Sustainable Wood Construction & Circular Economy",
            description:
                "TIMBERHAUS is a European innovation project dedicated to reducing the construction sector’s carbon footprint by developing climate-smart, circular solutions for wood construction. Addressing the fact that the building industry accounts for 40% of global CO₂ emissions and 35% of waste, TIMBERHAUS focuses on increasing the use of wood as a primary, sustainable material. The project is currently validating its technologies and strategies in three diverse pilot cities—Berlin (Germany), Baia Mare (Romania), and Siena (Italy)—to demonstrate how timber can be integrated into different urban contexts and heritage settings.",
            image: "./timberhaus.svg",
            link: "https://timberhaus.eu",
        },
        {
            title: "Ctfc - IoT Platform",
            description:
                "Our IoT platform collects and aggregates data from connected devices in real time, transforming it into clear, actionable insights. Sensor data is visualized directly on interactive floorplans, dashboards, and charts, making it easy to monitor environments, track performance, and identify issues at a glance. The platform centralizes complex device data into a simple, intuitive interface for smarter decision-making.",
            image: "./iot.jpg",
            link: "https://ctfc.innorenew.eu/grafana/public-dashboards/535d826acc744109b4d64a68654ee262?orgId=1&refresh=10s",
        },
    ];

    let posts: { id: number | null; title: string; body: string; email: string; created: string }[] = $state([]);
    let isLoggedIn = $state(false);

    onMount(async () => {
        isLoggedIn = (await verify()) === 200;
        const response = await fetch(`${PUBLIC_API_BASE_URL}/posts`);
        posts = await response.json();
        console.log(isLoggedIn);
    });

    const courses = [
        {
            title: "Information Technology for Sustainable Design - BIM with Elements of Algorithmics",
            description: "Module 1 (e-learning): The concept and workflow of the building information modeling (BIM); Introduction to the concept of algorithmics in relation to BIM processes; Exploring simulation tools for sustainable design.",
            url: "https://neb.academy/courses/information-technology-for-sustainable-design-bim-with-elements-of-algorithmics",
            tags: ["digital", "circular"],
            photo: "",
        },
        {
            title: "Urban Regeneration and Cultural Heritage Seminar",
            description: "During the course, selected cases of revitalization of historic areas will be discussed and analyzed, with particular emphasis on examples from Poland and Europe involving the revitalization of historic districts.",
            url: "https://neb.academy/courses/urban-regeneration-and-cultural-heritage-seminar",
            tags: ["bio-based", "circular", "regenerative"],
            photo: "",
        },
        {
            title: "Urban Regeneration",
            description: "During the course students are introduced to the practice of design in degraded urban structures, with particular emphasis on the principles of revitalization activities in historical areas.",
            url: "https://neb.academy/courses/urban-regeneration",
            tags: ["bio-based", "circular", "regenerative"],
            photo: "",
        },
        {
            title: "Sustainability Toolbox for Building Designers",
            description: "Sustainability Toolbox for Building Designers online course equips you with the tools needed to analyze and reduce the carbon footprint throughout a building's lifecycle. The course helps you lead the transition towards sustainable design.",
            url: "https://neb.academy/courses/sustainability-toolbox-for-building-designers",
            tags: ["bio-based", "long-life"],
            photo: "https://neb.academy/images/5/d/3/c/2/5d3c236a3f0063d4636a3fd915f5c0391c2f4e54-woodenhomes.webp",
        },
        {
            title: "Introduction to Wood Material",
            description: "Introduction to Wood Material is a set of two engaging online courses introducing learners to the world of wood. Wood remains one of the most important renewable materials available to humans.",
            url: "https://neb.academy/courses/introduction-to-wood-material",
            tags: ["bio-based", "circular", "long-life"],
            photo: "https://neb.academy/images/c/1/d/d/2/c1dd2488e4b74a28d08fb56ddd32a1abcd8982a0-wood.webp",
        },
        {
            title: "From Local Waste to Bio-based Innovation",
            description: "This course offers a practical, studio-based pathway for turning regional residues into high-performance, bio-based materials that advance a beautiful, sustainable, and inclusive built environment.",
            url: "https://neb.academy/courses/from-local-waste-to-bio-based-innovation",
            tags: ["bio-based", "circular", "bagaceira", "cfa"],
            photo: "https://neb.academy/images/d/3/7/f/4/d37f4fba271e3a8a8d9965aebc746656994a715d-bagaceira.jpg",
        },
        {
            title: "mini-MOOC: Designing Nature-Based Solutions With Circular and Bio-Sourced Materials",
            description: "This course aims to introduce city-makers to the foundational concepts of nature-based solutions (NbS) and material circularity, focusing on the potential of a bio-sourced material: mycelium, as a sustainable building resource.",
            url: "https://neb.academy/courses/mini-mooc",
            tags: ["bio-based", "circular", "iUE", "cfa"],
            photo: "https://neb.academy/images/e/1/8/2/1/e18215a2f9b74d7c1c9862cbeb297b9b9ad2ca84-iue.jpg",
        },
        {
            title: "Circular Design for Interiors: Sustainable Wall Finishes with Recycled Plaster",
            description: "This course introduces learners to sustainable and circular interior design principles through the practical lens of recycled plaster wall finishes. Through a video lecture, illustrated manuals, and design exercises.",
            url: "https://neb.academy/courses/volupte-course",
            tags: ["circular", "volupte", "cfa"],
            photo: "https://neb.academy/images/5/b/4/7/e/5b47e5247ed84a006c72b9c75fecef4926aa1fe2-voluptecourse.jpg",
        },
        {
            title: "Designing sustainable and friendly urban space for and with children",
            description: "Designing sustainable and friendly urban space for and with children. Jan Gehl, a Danish urbanist once said that most cities were designed for elephants, not for humans. Being city officials, planners, and designers, we can change that.",
            url: "https://neb.academy/courses/designing-sustainable-and-friendly-urban-space-for-and-with-children",
            tags: ["course", "beata-patuszynska", "cfa"],
            photo: "https://neb.academy/images/7/1/a/4/f/71a4f5631aaab4185576cdda29ae30f125b679b3-child.jpg",
        },
        {
            title: "Animating Local Heritage: Tools and Methods for Cultural Engagement",
            description: "The course 'Animating Local Heritage' aims to equip cultural practitioners, educators, and community animators with the practical tools and knowledge to design and implement local heritage-focused cultural engagement initiatives.",
            url: "https://neb.academy/courses/animating-local-heritage",
            tags: ["long-life", "digital", "heritage", "flaneur", "cfa"],
            photo: "https://neb.academy/images/0/2/a/4/8/02a481cdf83debd45cbb451e96379ecfaa3037b1-flaneurcourse.webp",
        },
        {
            title: "Green Construction",
            description: "Зелене будівництво як основа для екологічно чистого та кліматично стійкого будівництва (Green construction as a basis for environmentally friendly and climate-resilient building).",
            url: "https://neb.academy/courses/green-construction",
            tags: ["circular", "bdo", "knuba", "istu", "cfb"],
            photo: "https://neb.academy/images/9/d/b/9/5/9db95bf7d0e857eb34101d5e0a5a398e92cb1d2c-.jpg",
        },
        {
            title: "Municipal Facilities",
            description: "Екологічно чисті матеріали та інструменти енергоефективності в реконструкції муніципальних об'єктів (Eco-friendly materials and energy efficiency tools in the reconstruction of municipal facilities).",
            url: "https://neb.academy/courses/municipal-facilities",
            tags: ["circular", "bdo", "knuba", "istu", "cfb"],
            photo: "https://neb.academy/images/c/7/3/9/1/c739101e138a9c2e0c7b91a8edf50a8d22db0a4b-save-energy.jpg",
        },
        {
            title: "MOZAIK",
            description: "The NEBA MOZAIK Hub initiative focuses on sustainability, energy efficiency, and green business transformation with courses in Croatian and Serbian.",
            url: "https://neb.academy/courses/mozaik",
            tags: ["circular", "mozaik", "cfb"],
            photo: "https://neb.academy/images/a/4/5/c/d/a45cde9b0fac57afda132dce2c76e17aa9d458bd-gps-2.jpg",
        },
        {
            title: 'Extended Reality (XR) and "Phygital" Cities: Designing Hybrid Urban Experiences with Design Fiction',
            description: "This course introduces students to speculative and participatory design methods to critically explore the intersection of digital twins, smart cities, and Extended Reality (XR) technologies.",
            url: "https://neb.academy/courses/extended-reality-xr-and-phygital-cities-designing-hybrid-urban-experiences-with-design-fiction",
            tags: ["circular", "long-life", "regenerative", "digital", "karma", "cfb"],
            photo: "https://neb.academy/images/8/0/6/8/3/806836b074d8f0cd178619dd2d8114c51a62e113-karmacoursesplash.jpg",
        },
        {
            title: "Business Academy for Migrants",
            description: "This training program was developed in response to the urgent needs of Ukrainian migrants living in the Czech Republic. The majority of them are women with young children who face specific barriers to employment and entrepreneurship.",
            url: "https://neb.academy/courses/business-academy",
            tags: ["digital", "cfa", "inclusive", "podpora-podnikani", "course"],
            photo: "https://neb.academy/images/9/8/1/1/9/9811919c5461cafb0df53c7b8ca22d6c907515dc-centrumpro01.jpg",
        },
        {
            title: "Preservation Guidelines For Postwar Concrete Architecture",
            description: "Postwar concrete architecture represents a specific, abundant, and relatively recent yet significant facet of modern heritage, warranting a sustainable approach to its preservation. This training aims to provide guidelines for its conservation.",
            url: "https://neb.academy/courses/preservation-postwar-concrete-architecture",
            tags: ["long-life", "heritage", "vilnius-tech", "cfb"],
            photo: "https://neb.academy/images/d/1/4/c/8/d14c8d12e19a3cbfcb384c4074089b24bdab6756-sportorumailithuania.png",
        },
        {
            title: "Life Cycle Assessment For Building Preservation and Renovation",
            description: "This course provides a thorough understanding of Life Cycle Assessment (LCA) principles and methodologies, enabling learners to comprehensively assess the environmental impacts of building projects.",
            url: "https://neb.academy/courses/lca-building-preservation-and-renovation",
            tags: ["circular", "long-life", "vilnius-tech", "cfb"],
            photo: "https://neb.academy/images/8/3/0/5/9/830595275848918b57ea13e80f2e9819e540c718-lifecycleassessment.png",
        },
        {
            title: "From Biomass and Wool to Circular Construction Materials",
            description: "This course provides a practical understanding of how Nature-Based Carbonation can be accelerated and applied to manufacture Circular Construction Materials with waste such as electric arc furnace slag.",
            url: "https://neb.academy/courses/nature-based-carbonation",
            tags: ["bio-based", "circular", "ubi", "cfb"],
            photo: "https://neb.academy/images/6/8/2/2/a/6822a2882eec53a572593698dd373e449e199bc9-ubicarboncourse011.jpg",
        },
        {
            title: "Circular Resource Mapping: Identifying CO₂-Absorbing Waste",
            description: "This course offers conceptual and scientific foundations to demonstrate how regional bio-based residues can support sustainable and circular innovation in construction. By-products from agriculture, forestry and industry are explored.",
            url: "https://neb.academy/courses/circular-resource-mapping",
            tags: ["bio-based", "circular", "ubi", "cfb"],
            photo: "https://neb.academy/images/c/a/6/8/2/ca682760c485c0e8940ada858a7a64eb33d38618-circularresourcemapping.png",
        },
        {
            title: "Building with Biomass and Wool",
            description: "This is an in-person course which promotes learning through experimentation, collaborative work and interdisciplinarity. It brings together students, researchers, companies, associations and research institutes.",
            url: "https://neb.academy/courses/building-with-biomass-and-wool",
            tags: ["bio-based", "circular", "ubi", "cfb"],
            photo: "https://neb.academy/images/4/5/8/f/9/458f91ba34deee997a0a18e037c5b8fd59c16144-ubicarboncourse02.jpg",
        },
        {
            title: "Recovery of the building heritage through the NEB lens",
            description: "Ground-to-panel connection in refurbishment works on masonry buildings using CLT construction systems.",
            url: "https://neb.academy/courses/udine-2",
            tags: ["bio-based", "udine", "cfb"],
            photo: "https://neb.academy/images/4/7/e/5/c/47e5c8f4c80866055c294ef2cb8f011b89421d8b-udine-wall.jpg",
        },
        {
            title: "Ground connection in wooden buildings",
            description: "Ground-to-panel connection in refurbishment works on masonry buildings using CLT construction systems.",
            url: "https://neb.academy/courses/udine-3",
            tags: ["bio-based", "udine", "cfb"],
            photo: "https://neb.academy/images/3/7/6/a/e/376ae43f54357a49456689992a7b43b842f3d176-udine-ground.jpg",
        },
        {
            title: "Durability of timber cycle-pedestrian bridges",
            description: "From monitoring to design for durability of timber cycle-pedestrian bridges.",
            url: "https://neb.academy/courses/udine-1",
            tags: ["bio-based", "udine", "cfb"],
            photo: "https://neb.academy/images/b/8/4/f/7/b84f7c50c523003e4815031ec97e9879129dcd63-udine-footbridges.jpg",
        },
        {
            title: "Fostering Inclusive Urban Futures: Gender and Care Perspectives in City Making",
            description: "This course is designed for anyone interested in city making with a focus on gender and care perspectives, aiming to foster more inclusive urban environments. It welcomes architects, urban planners, policymakers and community advocates.",
            url: "https://neb.academy/courses/fostering-inclusive-urban-futures",
            tags: ["long-life", "inclusive", "urbana", "surf", "cfa"],
            photo: "https://neb.academy/images/4/a/8/8/d/4a88db92ab308593e1e848bba887bac76d3fd685-urbanacourse.jpg",
        },
        {
            title: "Training Biobased Design for Architects",
            description: "This training focuses on developing and disseminating knowledge and skills related to biobased design within architecture and engineering. The ultimate goal is a two-day intensive training for professionals.",
            url: "https://neb.academy/courses/training-biobased-design-for-architects",
            tags: ["bio-based", "nkbb", "cfa"],
            photo: "https://neb.academy/images/0/a/c/1/c/0ac1c13d2dd054d55176a7a0752b93d25d0d860e-biobasedbouwen.jpg",
        },
        {
            title: "Navigating Sustainability, Conservation & Circularity in the Historic Built Environment",
            description: "Design and delivery of a series of training modules on the interventions in the built environment, mapping current practices and case studies and testing them against sustainability, conservation, and circularity principles.",
            url: "https://neb.academy/courses/navigating-sustainability-conservation-and-circularity-in-the-historic-built-environment",
            tags: ["bio-based", "digital", "heritage", "cfa", "design", "buildings", "neb4bih"],
            photo: "https://neb.academy/images/6/e/1/a/2/6e1a2178ec88c013035b9aa051e6c543f18b6271-neb4bih.webp",
        },
        {
            title: "Seaweed as a Bio-based Construction Insulation",
            description: "A hands-on workshop exploring the potential of seaweed red algae (Gelidium corneum) as an insulation construction material, including risk management, specification/supply chain, lifecycle analysis, and building code compliance.",
            url: "https://neb.academy/courses/seaweed-as-a-bio-based-construction-insulation",
            tags: ["bio-based", "cfa", "atelier la juntana"],
            photo: "https://neb.academy/images/1/b/4/b/b/1b4bba647604aae15a582c4ea52916c8fe0a4169-project08.jpg",
        },
        {
            title: "Cultural Value-Community Needs-Based Methodology For Renovation of Educational Spaces",
            description: "Schools built decades ago no longer meet the fundamentally changed needs of education. This course addresses renovation goals beyond technical updates, focusing on cultural value and community needs.",
            url: "https://neb.academy/courses/renovation-of-educational-spaces",
            tags: ["long-life", "cfb", "inclusive", "vilnius-tech"],
            photo: "https://neb.academy/images/1/9/9/b/d/199bd152e9e930ae38b2511b323941995e5f9f1f-vilniustechcourse01.jpg",
        },
        {
            title: "Sustainable Building with Timber",
            description: "Innovations in architecture, engineering and manufacturing make it feasible to construct almost any building in timber, including high-rise structures. Discover how this way of building could help solve the climate crisis.",
            url: "https://neb.academy/courses/tu-delft-timber",
            tags: ["bio-based", "TU-Delft"],
            photo: "https://neb.academy/images/2/a/a/4/c/2aa4c5fc07c63fa2f1edf652940c968ee29bd251-timber.webp",
        },
        {
            title: "Modern Timber Construction",
            description: "This course introduces the fundamentals of modern timber construction, including key timber building systems, their advantages and limitations, and the specific characteristics of wood as a building material.",
            url: "https://neb.academy/courses/modern-timber-construction",
            tags: ["bio-based", "circular", "long-life"],
            photo: "https://neb.academy/images/2/8/1/2/3/281231580d45fe22ac504cc4713f715276c3db0c-modern-timber-contruction.webp",
        },
        {
            title: "Inclusive Urban Planning",
            description: "This course provides insights into how active inclusion and norm-critical perspectives can contribute to new approaches in urban planning and design. Participants learn how to create more attractive, equitable city spaces.",
            url: "https://neb.academy/courses/inclusive-urban-planning",
            tags: ["regenerative"],
            photo: "https://neb.academy/images/f/a/2/3/7/fa237fa0702936285c96cabc33d8585c5d0306cb-inclusive-urban-planning.webp",
        },
        {
            title: "Design Process for Green Transition",
            description: "This course provides a foundational understanding of methods and strategies for sustainable construction from an ecological perspective, with a focus on reducing climate impact. Participants gain tools for greener design processes.",
            url: "https://neb.academy/courses/design-process-for-green-transition",
            tags: ["circular", "long-life", "regenerative"],
            photo: "https://neb.academy/images/f/e/4/5/d/fe45dd0e6188be76cfc3e2a0afef87683ad559f6-designprocess-for-green-transition.webp",
        },
        {
            title: "Healthy Cities",
            description: "This course offers an in-depth exploration of the connections between urban design and human health and well-being. Participants are introduced to five key areas where research has demonstrated strong links between city form and health outcomes.",
            url: "https://neb.academy/courses/healthy-cities",
            tags: ["regenerative"],
            photo: "https://neb.academy/images/a/8/d/1/a/a8d1a8fae0d5b6c3bf272ed35056fd111c0a1701-healthy-cities.webp",
        },
        {
            title: "Wood First – Strategic Support for Timber Construction",
            description: "Wood First is a digital training program designed to support Swedish municipalities and regions in developing, updating, or following up on timber construction strategies or roadmaps for climate-neutral building.",
            url: "https://neb.academy/courses/wood-first-strategic-support-for-timber-construction",
            tags: ["bio-based"],
            photo: "https://neb.academy/images/9/b/0/3/4/9b0341e7ef640ec4c3ed8364a5dd5f1571da8745-woodfirst.webp",
        },
        {
            title: "Timber on Top – Digital Course on Vertical Timber Extensions",
            description: "This free, 30-minute digital course introduces the concept of vertical extensions using timber construction. It explores the opportunities for urban densification by adding new floors atop existing buildings.",
            url: "https://neb.academy/courses/timber-on-top-digital-course-on-vertical-timber-extensions",
            tags: ["bio-based"],
            photo: "https://neb.academy/images/a/e/2/4/d/ae24de53e28ff55201d20523d11c7ff5a91f7a4e-timber-on-top.webp",
        },
        {
            title: "CCBuild – Digital Course in Circular Construction",
            description: "This course introduces the fundamentals of circular economy principles within the construction and real estate sectors. It emphasizes the shift from traditional linear models to circular approaches for a sustainable built environment.",
            url: "https://neb.academy/courses/ccbuild-digital-course-in-circular-construction",
            tags: ["bio-based", "circular", "regenerative"],
            photo: "https://neb.academy/images/b/f/8/2/1/bf821d353c20ae86476efe58427b71164118d7c8-circular-construction.webp",
        },
        {
            title: "Circular Economy for a Sustainable Built Environment",
            description: "Learn how the principles of the Circular Economy can be applied to the built environment ranging from products and buildings to metropolitan and regional development strategies.",
            url: "https://neb.academy/courses/circular-economy-for-a-sustainable-built-environment",
            tags: ["TU-Delft", "circular"],
            photo: "https://neb.academy/images/f/5/b/4/3/f5b43896b6cc75b156b27be5cbf8469601414945-bkcesbe.webp",
        },
        {
            title: "Circularity Principles for Façade Engineering",
            description: "Discover how to integrate circularity into façade design. Gain insights into various circularity concepts and their practical applications in the façade design process, along with methods for assessing circularity performance.",
            url: "https://neb.academy/courses/circularity-principles-for-facade-engineering",
            tags: ["circular"],
            photo: "https://neb.academy/images/d/6/0/8/2/d6082e180ce92f86cd0598e3f10a31474f9f01c1-bkcpfe1x.webp",
        },
        {
            title: "Circular Building Products for a Sustainable Built Environment",
            description: "Be a frontrunner in circular building product development. Learn best practices and how to create value for your company in the transition to a Circular Economy. Suitable for architects, product manufacturers, and developers.",
            url: "https://neb.academy/courses/circular-building-products-for-a-sustainable-built-environment",
            tags: ["circular"],
            photo: "https://neb.academy/images/a/0/5/1/4/a0514aa330ea85a837b115553f37a0a288396b0d-bkcesbe2.webp",
        },
        {
            title: "Spatial Circularity Strategies for Sustainable Regional Development",
            description: "Gain insights and learn to use appropriate tools to develop spatial strategies, plans, and actions to support the transition toward circularity in your city or region.",
            url: "https://neb.academy/courses/spatial-circularity-strategies-for-sustainable-regional-development",
            tags: ["TU-Delft", "circular"],
            photo: "https://neb.academy/images/7/4/2/4/a/7424adc355b6597d20defc420d116ca42a3763ce-bkscsx.webp",
        },
        {
            title: "AI in Architectural Design: Introduction",
            description: "Learn about the elements of AI, machine learning and computer vision. Discover open-source AI tools and hidden AI gems. Learn how to play with architectural design data using freely available AI tools.",
            url: "https://neb.academy/courses/ai-in-architectural-design-introduction",
            tags: ["digital"],
            photo: "https://neb.academy/images/2/c/f/2/d/2cf2d1eb2fd7b3ed6185e9105d66a2cb59155bd6-bkaife2x.webp",
        },
        {
            title: "Capacity Building on Use of Wood for Buildings and Public Spaces",
            description: "These 8 webinars focus on the use of wood in the building sector, covering the characteristics, development, production, and utilization of wood-based construction materials and insights into regulations.",
            url: "https://neb.academy/courses/capacity-building-on-use-of-wood-for-buildings-and-public-spaces",
            tags: ["bio-based"],
            photo: "https://neb.academy/images/2/8/2/3/3/28233b5fc8aa49444e06921e5b2f40504ca87e5f-topic3.webp",
        },
        {
            title: "Capacity Building on Green and Circular Building",
            description: "These 9 webinars are focused on green and circular buildings, aiming to enhance understanding of sustainable construction. Key areas include environmental impacts, circularity in the construction sector, and best practices.",
            url: "https://neb.academy/courses/capacity-building-on-green-and-circular-building",
            tags: ["circular"],
            photo: "https://neb.academy/images/9/b/d/e/8/9bde8177cbb3418b94c887c06953b8a61401b10f-topic2.webp",
        },
        {
            title: "Professional Programme in Industrialised Wood Construction",
            description: "The programme's objective is to create an initial framework for entering the design and construction of industrialized buildings using wood, geared toward the various professionals who can participate in this sector.",
            url: "https://neb.academy/courses/professional-programme-in-industrialised-wood-construction",
            tags: ["bio-based", "long-life", "digital"],
            photo: "https://neb.academy/images/1/3/8/7/e/1387e699ea5193d2fd58f964f6b96bcbeaae1dbb-iaac.webp",
        },
        {
            title: "Introduction to Shipbuilding (All Hands on Deck online course)",
            description: "The Introduction to Shipbuilding course opens the doors of wooden boatbuilding to adult learners who already have woodworking expertise and want to expand it into new career paths.",
            url: "https://neb.academy/courses/introduction-to-shipbuilding-all-hands-on-deck-online-course",
            tags: ["bio-based", "long-life", "regenerative"],
            photo: "https://neb.academy/images/9/4/7/f/0/947f0fb89f0fa292f0d9bccb22e83489dc8b2a15-ahod3601600.jpg",
        },
        {
            title: "Capacity Building on NEB and Place Making",
            description: "These 7 webinars, called NEBinars, aim to cultivate a new professionalism — place-makers — starting with junior professionals as catalysts for change, emphasizing the principles outlined in the New European Bauhaus.",
            url: "https://neb.academy/courses/capacity-building-on-use-of-wood-for-buildings-and-public-spaces-copy",
            tags: ["regenerative"],
            photo: "https://neb.academy/images/f/1/f/0/2/f1f028f2c65ec8606a070292cf5d06f91a3e639c-topic1.webp",
        },
        {
            title: "NEB and Circular Economy Principles for Renovation to Prolong the Lifespan of Apartment Buildings",
            description: "This course focuses on the reuse of materials like construction and demolition waste in apartment building renovations according to NEB principles. Aimed at apartment association practitioners and renovation professionals.",
            url: "https://neb.academy/courses/neb-and-circular-economy-principles-for-renovation-to-prolong-the-lifespan-of-apartment-buildings",
            tags: ["digital", "heritage", "cfb", "course", "buildings"],
            photo: "https://neb.academy/images/5/f/3/e/e/5f3ee563f7cdfc3f94e5a55b94944f3ddfbfabfc-maja-tagant.webp",
        },
        {
            title: "Solutions for Renovation Supporting Sustainability Goals, Accessibility, and Residents' Well-being",
            description: "The course aims to equip practitioners in the non-profit apartment associations (HOAs) with the practical tools, knowledge and inspiration to make renovation of residential buildings more aesthetic, sustainable, and inclusive.",
            url: "https://neb.academy/courses/solutions-for-renovation-supporting-sustainability-goals-accessibility-and-residents-well-being",
            tags: ["cfb", "course", "design", "buildings"],
            photo: "https://neb.academy/images/0/e/5/8/8/0e588fcda046a0b3426b9fdec219fa3aa88593cf-valmis-hoone-majatagune.webp",
        },
        {
            title: "Renovation of Apartment Buildings According to the Principles of New European Bauhaus in Cooperation with the Community",
            description: "The course aims to equip practitioners in the non-profit apartment associations with the tools and knowledge to carry out renovation of residential buildings in cooperation with the community, following NEB principles.",
            url: "https://neb.academy/courses/renovation-of-apartment-buildings-according-to-the-principles-of-new-european-bauhaus-in-cooperation-with-the-community",
            tags: ["cfb", "course", "design", "buildings"],
            photo: "https://neb.academy/images/1/2/9/5/d/1295dcee2662642e3be0490f36734687896221b1-uafxx0tw.webp",
        },
        {
            title: "Design Thinking and Green Skills for Common Futures",
            description: "The course provides an interdisciplinary framework for exploring how design thinking and sustainability skills can be combined to address complex environmental and social challenges in the built environment.",
            url: "https://neb.academy/courses/design-thinking-and-green-skills-for-common-futures",
            tags: ["circular", "digital", "cfb", "course", "aks"],
            photo: "https://neb.academy/images/6/3/5/4/c/6354c6c70366ec89155ab0e7594bb0974547114a-cover-designthinkingandgreenskillsforcommonfutures-aks.webp",
        },
        {
            title: "From Idea to MVP: Building Sustainable Built-Environment Startups with Real-World Examples",
            description: "This course empowers aspiring entrepreneurs to transform their sustainable construction ideas into actionable Minimum Viable Products (MVPs). Through expert-led lectures, learners will grasp key startup methodologies applied to the built environment.",
            url: "https://neb.academy/courses/from-idea-to-mvp",
            tags: ["bio-based", "circular", "digital", "cfb", "socialtechlab"],
            photo: "https://neb.academy/images/b/b/6/d/1/bb6d158e61229a94efff96aba6e4b77daefc4447-stl.webp",
        },
    ];
</script>

<div class="w-[80%] px-6 py-12 bg-white/60 rounded-2xl text-gray-800 m-auto shadow-sm grid gap-10">
    <section>
        <h1 class="text-3xl font-bold mb-4">About</h1>
        <div class="grid gap-3 text-lg leading-relaxed">
            <b>Our platform is designed to accelerate research through intelligent, secure collaboration.</b>

            <div>We provide a shared digital environment where researchers can upload papers, datasets, reports, and working documents—transforming static files into a dynamic, searchable knowledge network.</div>
            <div>
                By converting uploaded research materials into rich semantic representations, our system understands the meaning and context behind the content, not just keywords. This enables researchers to ask natural-language questions about ongoing work and receive clear, contextual answers
                drawn directly from the collective research repository.
            </div>
        </div>
    </section>
    <div class="h-[1px] bg-black/10 w-full"></div>
    <section>
        <h1 class="text-3xl font-bold mb-4">Latest posts</h1>
        {#if posts.length > 0}
            <div class="grid gap-5 md:grid-cols-3 place-items-stretch">
                {#each posts as post}
                    <a target="_blank" href="/post/{post.id}" class=" bg-white rounded-lg border border-teal-800/10 overflow-hidden hover:shadow-lg transition-shadow duration-300">
                        <div class="p-6">
                            <div class="flex justify-between items-center">
                                <h2 class="text-xl text-accent mb-3 line-clamp-1">{post.title}</h2>
                                {#if isLoggedIn}
                                    <a href="/app/collaborate?id={post.id}" target="_blank">
                                        <MaskedIcon class="hover:bg-secondary bg-primary size-5" src="./edit.svg" />
                                    </a>
                                {/if}
                            </div>
                            <div class="text-gray-600 text-sm leading-relaxed line-clamp-2 w-full">{@html post.body}</div>
                            <div class="pt-5 flex justify-between items-center text-xs">
                                <span class="text-secondary">{post.email}</span><span class="text-secondary-1">{post.created}</span>
                            </div>
                        </div>
                    </a>
                {/each}
            </div>
        {:else}
            <div class="text-sm text-center text-accent">No posts yet.</div>
        {/if}
    </section>

    <div class="h-[1px] bg-black/10 w-full"></div>
    <section>
        <h2 class="text-2xl font-semibold mb-6">Visit our projects</h2>

        <div class="grid gap-10 grid-cols-1 md:grid-cols-4">
            {#each cards as card}
                <div class="bg-white rounded-lg shadow-sm overflow-hidden">
                    <img src={card.image} alt="Placeholder image" class="w-full h-[200px] object-cover border-2 rounded-lg shadow-xs border-white" />

                    <div class="p-4">
                        <h3 class="text-xl mb-1">
                            {card.title}
                        </h3>

                        <p class="text-sm mb-3 line-clamp-5">
                            {card.description}
                        </p>

                        <a href={card.link} target="_blank" rel="noopener noreferrer" class="text-sm text-blue-400 hover:underline font-medium">
                            <div class="flex gap-3">
                                <MaskedIcon src="./external-link.svg" class="bg-blue-400" />
                                Visit external page
                            </div>
                        </a>
                    </div>
                </div>
            {/each}
        </div>
    </section>
    <div class="h-[1px] bg-black/10 w-full"></div>
    <section>
        <h2 class="text-2xl font-semibold mb-6">Visit our courses</h2>
        <div class="grid md:grid-cols-3 gap-3">
            {#each courses as course}
                <a target="_blank" href={course.url} class="hover:shadow hover:brightness-95 bg-white rounded-xl p-5">
                    {#if course.photo}
                        <img src={course.photo} class="rounded-lg" />
                    {/if}
                    <div class="text-lg font-bold">{course.title}</div>
                    <div class="text-xs text-secondary-1">{course.tags.join(" ")}</div>
                    <div class="text-sm py-3 text-primary">{course.description}</div>
                </a>
            {/each}
        </div>
    </section>
</div>
