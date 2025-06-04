<script lang="ts">
    import { PUBLIC_API_BASE_URL } from "$env/static/public";
    import { onMount } from "svelte";
    import { fade, slide } from "svelte/transition";
    import { marked } from "marked";
    import MaskedIcon from "$lib/common/MaskedIcon.svelte";
    import type { ResultChunk } from "$lib/types/question";

    let data: string = $state("");
    let query: string = $state("");
    let waiting = $state(false);
    let component: HTMLDivElement | undefined = $state(undefined);
    let maxHeight = $derived((component?.clientHeight ?? 100) * 0.8);
    let chunks: ResultChunk[] = $state([]);

    async function sendQuery() {
        data = "";
        chunks = [];
        waiting = true;
        console.log(`${PUBLIC_API_BASE_URL}/api/search?query=${query}`);
        const response = await fetch(`${PUBLIC_API_BASE_URL}/api/search?query=${query}`);
        const stream = response.body?.getReader();

        if (!stream) {
            console.error("idk bro ");
            return;
        }
        const decoder = new TextDecoder("utf-8");
        let total = 0;
        while (true) {
            const { done, value } = await stream?.read();
            const decoded = decoder.decode(value);
            total++;
            if (total === 1) {
                chunks = JSON.parse(decoded);
                continue;
            }
            if (done) {
                console.log("Done");
                break;
            }

            data += decoded;
        }
        waiting = false;
    }

    const sample_json = [
        {
            id: 'PointId { point_id_options: Some(Uuid("c69c3914-12f8-4d43-be0e-08dbf0d415a5")) }',
            doc_id: "4",
            doc_seq_num: 111,
            content:
                "until constant mass was obtained (duration approximately 1-2 weeks). Tensile properties were determined using a universal testing machine (ZwickRoell, Germany) equipped with a 100 kN load cell at 20 ˚C and 65% RH in accordance to ISO 527 with an initial clamp distance of 46 mm and a crosshead speed of 5 mm/min. The change in length was measured with a contact extensometer (ZwickRoell, Germany) with an initial length of 20 mm. Three-point bending measurements were used to determine the bending properties of the DWRPs using the universal testing machine equipped with a 1 kN load cell with a crosshead speed of 4 mm/min. In accordance with ASTM D790, the span between the loading supports was set to 80 mm, which results in a span-to-thickness ratio of approximately 32, and the radii of the loading nose and loading supports were chosen to be 5 mm and 2 mm, respectively. *Microscopy.* Light microscope imaging (Olympus BX51, Japan), scanning electron microscopy (SEM) imaging (Hitachi SU5000, Japan) and atomic force microscopy (AFM) imaging (NanoWizard 4, JPK Instruments AG, Germany) were conducted in order to investigate the infiltration and densification behavior. Cross section cuts as well as longitudinal cuts (radial) were analyzed by light microscopy and SEM. DWRPs were embedded in epoxy potting resin (EpoFix, Struers GmbH, Germany) and the embedded samples were polished after curing using a LaboPol-25 (Struers GmbH, Germany). High quality surfaces for AFM imaging were obtained using an ultramicrotome (Ultracut, Reichert-Jung, Germany) with a diamond trim knife (Diatome, Switzerland). AFM",
            additional_data: "*Testing Conditions**",
            score: 0.5870669,
        },
        {
            id: 'PointId { point_id_options: Some(Uuid("f4139573-3b29-462b-978e-def6b6a15a8a")) }',
            doc_id: "23",
            doc_seq_num: 7,
            content:
                "set. Figure 3a and b exemplarily show the cell tissues of unmodified samples with different compression sets. Obviously, the stronger densification causes more damages. The cells of the 50% densified sample (Figure 3b) suffered considerably more cracks in the cell walls than the cells of the 30% densified sample (Figure 3a). Because of the cracks, a higher radial extension is possible by swelling, which leads to a higher swelling-shrinkage movement. The only difference between the two swelling coefficient graphs of unmodified densified samples is the initial swelling, which includes the set recovery. The set recovery comprises the decrease of plastic deformation of cells (i.e., the approach of cell and lumen shape to their original shape). In the case of WST, fluid water is soaked into the lumens due to releasing vacuum. Fluid water can immediately loosen bonds between fibrils, which were fixed during plastic deformation. Hence, the lumens are opened and reshaped. Thus, cell structure is recovered to a considerably higher degree than it was possible with the water vapor alone available at 90% RH. The behavior of furfurylated samples is inconsistent in the case of WST and ACT. Samples with 30% densification behave like unmodified densified samples. The ACT results in lower *RT,max* (5%) compared with WST (8–10%). The alternating climate also causes a lower swelling-shrinkage movement of 5% to 6%, whereas the ![](_page_3_Figure_2.jpeg) #### **Figure 3** Incident light microscope images before moisture exposure. (a) Unmodified cell tissue, 30% densified; (b) unmodified cell tissue, 50% densified; and (c) furfurylated",
            additional_data: '"What are the two types of testing methods compared in this study?"',
            score: 0.5675371,
        },
        {
            id: 'PointId { point_id_options: Some(Uuid("63414f6b-ebbd-4e71-b929-6dd201770547")) }',
            doc_id: "23",
            doc_seq_num: 12,
            content:
                "colleagues in this institution for the generous support. Received March 25, 2013; accepted June 3, 2013; previously published online June 21, 2013 - Kamke, F.A., Rathi, V.M. (2011) Apparatus for viscoelastic thermal compression of wood. Eur. J. Wood Prod. 69:483–487. - Kutnar, A., Kamke, F.A. (2012) Influence of temperature and steam environment on set recovery of compressive deformation of wood. Wood Sci. Technol. 46:953–964. - Laine, K., Rautkari, L., Hughes, M. (2013) The effect of process parameters on the hardness of surface densified Scots pine solid wood. Eur. J. Wood Prod. 71:13–16. - Lande, S., Westin, M., Schneider, M. (2004) Properties of furfurylated wood. Scand. J. For. Res. 19:22–30. - Morsing, N. (2000) Densification of Wood. Dissertation at Technical University of Denmark. - Navi, P., Girardet, F. (2000) Effects of thermo-hydro-mechanical treatment on the structure and properties of wood. Holzforschung 54:287–293. - Pfriem, A., Dietrich, T., Buchelt, B. (2012) Furfuryl alcohol impregnation for improved plasticization and fixation during the densification of wood. Holzforschung 66:215–218. - Rautkari, R., Kamke, F.A., Hughes M. (2011) Density profile relation to hardness of viscoelastic thermal compressed (VTC) wood composite. Wood Sci. Technol. 45:693–705. - Seborg, R., Millet, M., Stamm, A. (1945) Heat stabilized compressed wood – Staypak. Mech. Eng. 67:25–31. - Skyba, O., Niemz, P., Schwarze, F.W.M.R. (2008) Degradation of thermo-hygro-mechanically (THM)-densified wood by soft-rot fungi. Holzforschung 62:277–283. - Skyba, O., Niemz, P., Schwarze, F.W.M.R. (2009) Resistance of thermo-hygro-mechanically (THM) densified wood to degradation by white rot fungi. Holzforschung 63:639–646.",
            additional_data: "What are the two testing methods compared in this study?",
            score: 0.5498441,
        },
        {
            id: 'PointId { point_id_options: Some(Uuid("d6af38fe-86b8-4fd0-89f8-b1cc89c9cd6f")) }',
            doc_id: "23",
            doc_seq_num: 9,
            content:
                "disappear. Thus, water can hardly penetrate via soaking (i.e., penetration is inhibited by furfurylation and swelling hardly occurs). The situation is different in the case of ACT. Twenty weeks passed until the 50% densified samples reached an EMC, whereas the 30% densified samples have been conditioned within 13 weeks. This fact also proves that the water accessibility is easier with a lower compression set, but this fact also shows that EMC can be reached with furfurylated 50% densified wood. However, more time is necessary for this. It can be stated that a complete swelling of furfurylated 50% densified wood requires longer water storage times. Figure 4 shows the spring-back of the samples obtained from WST (a) and ACT (b). Regarding the unmodified densified samples, the distinctly higher spring-back due to water storage becomes obvious. The spring-back in the case of WST amounts to 82% to 84% for both 30% and 50% densified samples. Storage in ACT causes a springback of 50% to 55% for 30% densified samples and 41% to 48% for 50% densified samples. The reason for this behavior is the same as described above. The spring-back increases under ACT conditions with increasing cycle number. This means that the more frequently the humidity is alternated, the higher is the densification recovery. The furfurylated samples do not behave in that way. The spring-back of WST samples amounts to 3% for 30% densified samples and 0% for 50% densified samples. Storage in ACT conditions causes a spring-back of 2% to 1%",
            additional_data: "What are the two testing methods compared in this study?",
            score: 0.5498441,
        },
        {
            id: 'PointId { point_id_options: Some(Uuid("ea327a9d-66e8-49be-98a6-de099ac088f8")) }',
            doc_id: "28",
            doc_seq_num: 14,
            content:
                '(ASTM) (2004) Standard test methods for small clear specimens of timber. D-143-94. Vol. 04.10 on Wood. Annual Book of ASTM Standards, ASTM. West Conshohocken, PA - Bengtsson C, Jermer J, Brem F (2002) Bending strength of heattreated spruce and pine timber. IRG/WP 02-40242 - Callister WD (1994) Materials Science and Engineering: An Introduction. John Wiley and Sons, New York - Chanrion P, Schreiber J (2002) Bois Traite par Haute Temp ´ erature. ´ CTBA, France - Dirol D, Guyonnet R (1993) The improvement of wood durability by retification process. IRG/WP 98-40015 - Finnish ThermoWood Association (2003) ThermoWood Handbook. Helsinki, Finland - Kotilainen R (2000) Chemical changes in wood during heating at 150–260 ◦C. Ph.D. Thesis, Jyvaskyl ¨ a University, Finland ¨ - Militz H (2002) Thermal treatment of wood: European processes and their background. IRG/WP 02-40241 - Rapp AO, Sailer M (2000) Heat treatment in Germany. Proceedings of Seminar "Production and development of heat treated wood in Europe", Helsinki, Finland - Sailer M, Rapp AO, Leithoff H (2000) Improved resistance of Scots pine and spruce by application of an oil-heat treatment. IRG/WP 00-40162 - Santos JA (2000) Mechanical behaviour of Eucalyptus wood modified by heat. Wood Sci Technol 34:39–43 - Syrjanen T, Kangas E (2000) Heat treated timber in Finland. IRG ¨ /WP 00-40158 - Tjeerdsma BF, Boonstra M, Militz H (1998a) Thermal modification of non-durable wood species. IRG/WP 98-40124 - Tjeerdsma BF, Boonstra M, Pizzi A, Tekely P, Militz H (1998b) Characterisation of thermally modified wood: molecular reasons for',
            additional_data: "Stiffness",
            score: 0.54398847,
        },
        {
            id: 'PointId { point_id_options: Some(Uuid("c44cf93d-bca5-46e9-9efe-0734bc4e4c56")) }',
            doc_id: "23",
            doc_seq_num: 6,
            content:
                "can be well explained with the water availability. Water saturation of cell walls is independent of the physical state of water (fluid or gaseous). Concerning the unmodified densified samples, the *RT* of WST samples is distinctly higher than of those of ACT. The swelling coefficients of WST samples amount to 36% (*c* = 30%) and 73% to 75% (*c* = 50%) and those of ACT samples amount to 25% (*c* = 30%) and 41% to 44% (*c* = 50%). The available amounts and phases of water (water vapor at 90% RH and fluid water) result in different saturation degrees of the cell walls, but the significant differences with this regard cannot be explained ![](_page_1_Figure_19.jpeg) **Figure 1** Illustration of the nomenclature for thicknesses in Eqs. (2) and (3). ![](_page_2_Figure_2.jpeg) **Figure 2** Swelling coefficients of (a) WSTs and (b) ACTs. solely by this fact. On the contrary, the extent of swelling caused by the different methods (i.e., the swellingshrinkage movement) does not differ significantly. The unmodified 30% densified WST samples have a range of swelling-shrinkage movement of 11% (after the initial swelling), and 9% to 10%, under conditions of ACT. The unmodified 50% densified samples have a swelling range of 17% and 15% to 16% for WST and ACT, respectively. An interesting fact is the dependence on swellingshrinkage movement of the compression set. Figure 3a and b exemplarily show the cell tissues of unmodified samples with different compression sets. Obviously, the stronger densification causes more damages. The cells of the 50% densified",
            additional_data: "What are the two primary testing methods discussed in the passage?",
            score: 0.5387249,
        },
        {
            id: 'PointId { point_id_options: Some(Uuid("48989784-f48b-46ed-9cd9-ec343da8ef91")) }',
            doc_id: "23",
            doc_seq_num: 3,
            content:
                "polymer matrix of the cell wall and fixates its densified shape. One of the objectives of this study is to detect the influence of the test method on the results of swelling parameters, spring-back, and set recovery. ### **Materials and methods** Beech wood (*Fagus sylvatica* L.) with dimensions of 30 mm (T), 20 mm (R), and 80 mm (L) were tested. Furfurylation was conducted according to Pfriem et al. (2012). The samples were impregnated with a solution of furfuryl alcohol and with 5% maleic anhydride as catalyst. Then, the samples were densified in a hot press at 150°C and left in the press for 1 h. A 24 h post-curing process at 103°C completed the modification process. Control samples (without modification and densification) were also considered. The compression set (*c*) was performed to 30% and 50%: $c(\\%)$=100($T_{in}$-$T_{o}$)/$T_{in}$ where *Tin* and *T0* are the dimensions in densification direction (R direction) at room temperature before and after compression. Each test series comprised six samples. Table 1 shows the average densities (oven-dried) of the test series. The samples were cut into two parts: one half was used for WSTs and the other half for ACTs. For evaluation of cell damages due to densification, images were taken with incident light microscopy. To this end, samples were prepared before and after moisture exposure. The steps of the WSTs were (1) oven-drying, (2) pressure reduction (0.1 bar) for 30 min, (3) addition of 20°C warm water with simultaneous vacuum release, (4) storage of the samples in",
            additional_data: "What are the two primary testing methods mentioned in the passage?",
            score: 0.533162,
        },
        {
            id: 'PointId { point_id_options: Some(Uuid("3c004a51-efce-45e0-be88-4b7110c71453")) }',
            doc_id: "7",
            doc_seq_num: 22,
            content:
                "set up was as per ASTM-D4475-02:2016 [20] and yield moment capacity was calculated using the following expression [28]: $${\\rm M}_{\\rm y,eff}=\\frac{3}{8}{\\rm F}_{\\rm y}.{\\rm d}\\tag{2}$$ where Fy is the yield load (N) and d is the dowel diameter (mm). The yield moment capacity of the CW dowels was higher than the normal beechwood dowels in each tested grain angle (0◦, 45◦ and 90◦). As it can be seen in Fig. 10, the yield moment capacity of CW dowels in radial, tangential directions and at 45◦ angle were 1.15, 2.28 and 1.42 times higher than the normal beechwood dowels tested in the corresponding directions. The beech dowels loaded in the tangential direction ![](_page_7_Figure_1.jpeg) **Fig. 10.** Average load–displacement curves of yield moment tests. showed the highest ductility, whereas this was least for CW dowels loaded in this direction. For CW dowels, the yield moment capacity increased as the angle between the load and growth rings increased (from 0◦ to 90◦). Based on the results, it can be concluded that the angle between the orientation of growth rings of CW dowels and loading direction is a decisive factor in determining the load-carrying capacity and ductility of the dowel. The literature review showed scarce availability of data on yield moment capacity of the CW dowels. For this reason, the results of the current study were compared with calculated (using Equation 8.30 of EC5 [27]) yield moment capacity of S235 grade (ultimate tensile strength of 360 MPa) steel dowels of 10 mm diameter. The yield moment",
            additional_data: "What is the standard used for setting up the test mentioned in the passage?",
            score: 0.5267639,
        },
        {
            id: 'PointId { point_id_options: Some(Uuid("48124842-adba-4f9e-bc63-dbdbb4e8ef3e")) }',
            doc_id: "7",
            doc_seq_num: 31,
            content:
                "handbook wood as an engineering material chapter 11. Centennial ed. General technical report, 2010. - [7] V. Hemmil¨ a, S. Adamopoulos, O. Karlsson, A. Kumar, Development of sustainable bio-adhesives for engineered wood panels–a review, RSC Adv. 7 (2017) 38604–38630. - [8] P. Navi, D. Sandberg, Thermo-Hydro-mechanical wood processing, EPFL Press (2012). - [9] P. Haller, J. Wehsener, Festigkeitsuntersuchungen an Fichtenpressholz (FPH) Mechanical properties of densified spruce, Holz als Roh-und Werkstoff 62 (6) (2004) 452–454. - [10] A. Kutnar, D. Sandberg, P. Haller, Compressed and moulded wood from processing to products, Holzforschung 69 (7) (2015) 885–897. - [11] J.U. Hartig, J. Wehsener, P. Haller, Experimental and theoretical investigations on moulded wooden tubes made of beech (Fagus sylvatica L.), Constr. Build. Mater. 126 (2016) 527–536. - [12] C.R. Welzbacher, J. Wehsener, A.O. Rapp, P. Haller, Thermo-mechanische Verdichtung und thermische Modifikation von Fichtenholz (Picea abies Karst) im industriellen Maßstab – Betrachtung der Dimensionsstabilit¨ at und Dauerhaftigkeit 66 1 2008 39 49. - [13] I. El-Houjeyri, V.-D. Thi, M. Oudjene, M. Khelifa, Y. Rogaume, A. Sotayo, Z. Guan, Experimental investigations on adhesive free laminated oak timber beams and timber-to-timber joints assembled using thermo-mechanically compressed wood dowels, Constr. Build. Mater. 222 (2019) 288–299. - [14] A. Sotayo, D. Bradley, M. Bather, S. Namari, P. Sareh, P. Haller, M. Oudjene, A. Harte, S. Mehra, I. El-Houjeyri and Z. Guan, Review of state of the art of dowel laminated timber members and densified wood materials as sustainable engineered wood products for construction and building applications, Developments in",
            additional_data: '"What is the primary goal of these tests?"',
            score: 0.5236217,
        },
    ];

    // chunks = sample_json;
</script>

<div class="flex flex-wrap sm:grid grid-cols-2 h-full gap-5">
    <div class="p-5 border border-secondary/50 shadow-md rounded-lg bg-light-background w-full">
        <form class="flex gap-5 items-center">
            <input bind:value={query} type="text" class="w-full rounded-lg py-2 px-4 border border-secondary" placeholder="Ask a question ..." />
            <button type="submit" onclick={sendQuery} class="flex items-center gap-2 border flex-1 px-4 py-2 rounded-lg bg-secondary/10 border-secondary hover:brightness-75">
                <MaskedIcon src="../contact.svg" class="size-3 bg-secondary" />
                Ask
            </button>
        </form>
        {#if chunks.length > 0}
            <div class="pt-5 text-gray-500">Data retrieved from files:</div>
        {/if}

        <ul class="spacing-y-5 py-5">
            {#each chunks.slice(0, 5) as chunk, i}
                <li class="bg-secondary/5 group shadow-secondary/30 border-secondary/30 mb-5 hover:border-accent hover:shadow-accent/50 hover:bg-accent/5 border p-3 rounded-lg" in:slide={{ delay: i * 1000 }}>
                    <div>
                        <div class="flex justify-between items-center bg-white py-2 px-5 rounded-lg border">
                            <div class="flex gap-2">
                                <div class="text-xs bg-secondary/5 px-2 rounded border-secondary/50 border group-hover:border-accent/50 group-hover:bg-accent/5 group-hover:text-accent">#{i + 1}</div>
                                <div>{chunk.additional_data}</div>
                            </div>
                            <button disabled class="disabled:opacity-50 bg-gray-50 border py-1 px-2 rounded flex items-center gap-2">
                                <MaskedIcon src="../download.svg" class="size-3 bg-secondary group-hover:bg-accent/50" />
                                Download
                            </button>
                        </div>
                        <div class="text-wrap text-xs p-2 truncate bg-light-background border rounded-lg mt-3">
                            <div class="uppercase text-gray-400">Preview</div>
                            <div class="response preview p-3 prose-sm text-xs">
                                <div class="">{@html marked("... " + chunk.content.slice(0, 3000) + " ...")}</div>
                            </div>
                        </div>
                    </div>
                </li>
            {/each}
        </ul>
    </div>
    <div id="llm" class=" p-10 border border-secondary/50 shadow-md rounded-lg w-full" bind:this={component}>
        {#if data && data.length >= 0}
            <div in:fade>
                <div class="opacity-30">Woody's response</div>
                <div class="overflow-auto p-5 flex flex-col-reverse">
                    <div class="response preview spacing-y-2 prose-sm">{@html marked(data)}</div>
                </div>
            </div>
        {:else if waiting}
            <div class="flex items-center justify-center gap-5">
                <MaskedIcon src="../loading.svg" class="size-3 bg-secondary animate-spin" />
                Waiting for data!
            </div>
        {:else}
            <div class="text-center text-gray-500">You haven't asked or queried for anything yet.</div>
        {/if}
    </div>
</div>

<style>
    .response :global(li) {
        padding: 0.25rem 0;
        transition: all 0.2s ease-in-out;
    }
</style>
