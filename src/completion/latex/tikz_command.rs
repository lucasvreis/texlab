use crate::completion::factory;
use crate::completion::factory::LatexComponentId;
use crate::completion::latex::combinators::LatexCombinators;
use crate::feature::FeatureRequest;
use lsp_types::{CompletionItem, CompletionParams};
use std::borrow::Cow;

pub struct LatexTikzCommandCompletionProvider;

impl LatexTikzCommandCompletionProvider {
    pub async fn execute(request: &FeatureRequest<CompletionParams>) -> Vec<CompletionItem> {
        await!(LatexCombinators::command(request, async move |_| {
            let id = LatexComponentId::User(vec!["tikz.sty".to_owned()]);
            if request
                .component_database
                .related_components(&request.related_documents)
                .iter()
                .any(|component| component.files.iter().any(|file| file == "tikz.sty"))
            {
                COMMANDS
                    .iter()
                    .map(|name| factory::create_command(Cow::from(*name), &id))
                    .collect()
            } else {
                Vec::new()
            }
        }))
    }
}

const COMMANDS: &'static [&'static str] = &[
    "afterdecoration",
    "anchor",
    "anchorborder",
    "arrow",
    "arrowreversed",
    "attribute",
    "beginpgfgraphicnamed",
    "behindbackgroundpath",
    "behindforegroundpath",
    "backgroundpath",
    "beforebackgroundpath",
    "beforedecoration",
    "beforeforegroundpath",
    "breakforeach",
    "clip",
    "calendar",
    "chainin",
    "colorcurrentmixin",
    "coordinate",
    "datavisualization",
    "decoration",
    "deferredanchor",
    "draw",
    "endpgfgraphicnamed",
    "fill",
    "filldraw",
    "foreach",
    "foregroundpath",
    "graph",
    "inheritanchor",
    "inheritanchorborder",
    "inheritbackgroundpath",
    "inheritbeforebackgroundpath",
    "inheritbeforeforegroundpath",
    "inheritbehindbackgroundpath",
    "inheritbehindforegroundpath",
    "inheritforegroundpath",
    "inheritsavedanchors",
    "ifdate",
    "ifpgfallowupsidedowattime",
    "ifpgfgdgraphdrawingscopeactive",
    "ifpgfmathmathunitsdeclared",
    "ifpgfmathunitsdeclared",
    "ifpgfrememberpicturepositiononpage",
    "ifpgfresetnontranslationsattime",
    "ifpgfslopedattime",
    "jobname",
    "matrix",
    "method",
    "n",
    "node",
    "nodepart",
    "nodeparts",
    "pattern",
    "p",
    "path",
    "pgfactualjobname",
    "pgfaliasimage",
    "pgfalternateextension",
    "pgfapproximatenonlineartransformation",
    "pgfapproximatenonlineartranslation",
    "pgfarrowsaddtolateoptions",
    "pgfarrowsaddtolengthscalelist",
    "pgfarrowsaddtooptions",
    "pgfarrowsaddtowidthscalelist",
    "pgfarrowshullpoint",
    "pgfarrowslengthdependent",
    "pgfarrowslinewidthdependent",
    "pgfarrowssave",
    "pgfarrowssavethe",
    "pgfarrowssetbackend",
    "pgfarrowssetlineend",
    "pgfarrowssettipend",
    "pgfarrowssetvisualbackend",
    "pgfarrowssetvisualtipend",
    "pgfarrowsthreeparameters",
    "pgfarrowsupperhullpoint",
    "pgfcalendar",
    "pgfcalendardatetojulian",
    "pgfcalendarifdate",
    "pgfcalendarjuliantodate",
    "pgfcalendarjuliantoweekday",
    "pgfcalendarmonthname",
    "pgfcalendarmonthshortname",
    "pgfcalendarshorthand",
    "pgfcalendarsuggestedname",
    "pgfcalendarweekdayname",
    "pgfcalendarweekdayshortname",
    "pgfcoordinate",
    "pgfcurvilineardistancetotime",
    "pgfdata",
    "pgfdeclarearrow",
    "pgfdeclaredataformat",
    "pgfdeclaredecoration",
    "pgfdeclarefading",
    "pgfdeclarefunctionalshading",
    "pgfdeclarehorizontalshading",
    "pgfdeclareimage",
    "pgfdeclarelayer",
    "pgfdeclarelindenmayersystem",
    "pgfdeclaremask",
    "pgfdeclaremetadecorate",
    "pgfdeclarepatternformonly",
    "pgfdeclarepatterninherentlycolored",
    "pgfdeclareplothandler",
    "pgfdeclareplotmark",
    "pgfdeclareradialshading",
    "pgfdeclareshape",
    "pgfdeclareverticalshading",
    "pgfdecorateaftercode",
    "pgfdecoratebeforecode",
    "pgfdecoratecurrentpath",
    "pgfdecoratedangle",
    "pgfdecoratedcompleteddistance",
    "pgfdecoratedinputsegmentcompleteddistance",
    "pgfdecoratedinputsegmentlength",
    "pgfdecoratedinputsegmentremainingdistance",
    "pgfdecoratedpath",
    "pgfdecoratedpathlength",
    "pgfdecoratedremainingdistance",
    "pgfdecorateexistingpath",
    "pgfdecoratepath",
    "pgfdecorationpath",
    "pgfdvdeclarestylesheet",
    "pgferror",
    "pgfextra",
    "pgfextractx",
    "pgfextracty",
    "pgfgdaddspecificationhook",
    "pgfgdbegineventgroup",
    "pgfgdbeginlayout",
    "pgfgdbeginscope",
    "pgfgdedge",
    "pgfgdendeventgroup",
    "pgfgdendlayout",
    "pgfgdendscope",
    "pgfgdevent",
    "pgfgdeventgroup",
    "pgfgdsetedgecallback",
    "pgfgdsetlatenodeoption",
    "pgfgdsetrequestcallback",
    "pgfgdsubgraphnode",
    "pgfgetlastxy",
    "pgfgettransform",
    "pgfgettransformentries",
    "pgfhorizontaltransformationadjustment",
    "pgfimage",
    "pgfintersectionofpaths",
    "pgfintersectionsolutions",
    "pgfintersectionsortbyfirstpath",
    "pgfintersectionsortbysecondpath",
    "pgfkeys",
    "pgfkeysactivatefamilies",
    "pgfkeysactivatefamiliesandfilteroptions",
    "pgfkeysactivatefamily",
    "pgfkeysactivatesinglefamilyandfilteroptions",
    "pgfkeysalso",
    "pgfkeysalsofiltered",
    "pgfkeysalsofilteredfrom",
    "pgfkeysalsofrom",
    "pgfkeysdeactivatefamily",
    "pgfkeysdef",
    "pgfkeysdefargs",
    "pgfkeysdefnargs",
    "pgfkeysedef",
    "pgfkeysedefargs",
    "pgfkeysedefnargs",
    "pgfkeysevalkeyfilterwith",
    "pgfkeysfiltered",
    "pgfkeysgetfamily",
    "pgfkeysgetvalue",
    "pgfkeysifdefined",
    "pgfkeysiffamilydefined",
    "pgfkeysinstallkeyfilter",
    "pgfkeysinstallkeyfilterhandler",
    "pgfkeysisfamilyactive",
    "pgfkeyslet",
    "pgfkeyssavekeyfilterstateto",
    "pgfkeyssetfamily",
    "pgfkeyssetvalue",
    "pgfkeysvalueof",
    "pgflibraryfpuifactive",
    "pgflindenmayersystem",
    "pgflinewidth",
    "pgflowlevel",
    "pgflowlevelobj",
    "pgflowlevelsynccm",
    "pgflsystemcurrentleftangle",
    "pgflsystemcurrentrightangle",
    "pgflsystemcurrentstep",
    "pgflsystemdrawforward",
    "pgflsystemmoveforward",
    "pgflsystemrandomizeleftangle",
    "pgflsystemrandomizerightangle",
    "pgflsystemrandomizestep",
    "pgflsystemrestorestate",
    "pgflsystemsavestate",
    "pgflsystemturnleft",
    "pgflsystemturnright",
    "pgfmathabs",
    "pgfmathacos",
    "pgfmathadd",
    "pgfmathaddtocount",
    "pgfmathaddtocounter",
    "pgfmathaddtolength",
    "pgfmathand",
    "pgfmathanglebetweenlines",
    "pgfmathanglebetweenpoints",
    "pgfmathapproxequalto",
    "pgfmatharray",
    "pgfmathasin",
    "pgfmathatan",
    "pgfmathatantwo",
    "pgfmathbasetoBase",
    "pgfmathbasetobase",
    "pgfmathbasetodec",
    "pgfmathbin",
    "pgfmathceil",
    "pgfmathcos",
    "pgfmathcosec",
    "pgfmathcosh",
    "pgfmathcot",
    "pgfmathdeclarefunction",
    "pgfmathdeclarerandomlist",
    "pgfmathdectoBase",
    "pgfmathdectobase",
    "pgfmathdeg",
    "pgfmathdepth",
    "pgfmathdiv",
    "pgfmathdivide",
    "pgfmathe",
    "pgfmathequal",
    "pgfmathexp",
    "pgfmathfactorial",
    "pgfmathfalse",
    "pgfmathfloat",
    "pgfmathfloatabserror",
    "pgfmathfloatcreate",
    "pgfmathfloatgetexponent",
    "pgfmathfloatgetflags",
    "pgfmathfloatgetflagstomacro",
    "pgfmathfloatgetmantissa",
    "pgfmathfloatgetmantissatok",
    "pgfmathfloatifapproxequalrel",
    "pgfmathfloatifflags",
    "pgfmathfloatint",
    "pgfmathfloatlessthan",
    "pgfmathfloatmultiplyfixed",
    "pgfmathfloatparsenumber",
    "pgfmathfloatqparsenumber",
    "pgfmathfloatrelerror",
    "pgfmathfloatround",
    "pgfmathfloatroundzerofill",
    "pgfmathfloatsetextprecision",
    "pgfmathfloatshift",
    "pgfmathfloattoextentedprecision",
    "pgfmathfloattofixed",
    "pgfmathfloattoint",
    "pgfmathfloattomacro",
    "pgfmathfloattoregisters",
    "pgfmathfloattoregisterstok",
    "pgfmathfloattosci",
    "pgfmathfloatvalueof",
    "pgfmathfloor",
    "pgfmathfrac",
    "pgfmathgcd",
    "pgfmathgeneratepseudorandomnumber",
    "pgfmathgreater",
    "pgfmathheight",
    "pgfmathHex",
    "pgfmathhex",
    "pgfmathifisint",
    "pgfmathifthenelse",
    "pgfmathint",
    "pgfmathiseven",
    "pgfmathisodd",
    "pgfmathisprime",
    "pgfmathless",
    "pgfmathln",
    "pgfmathlog",
    "pgfmathlogten",
    "pgfmathlogtwo",
    "pgfmathmax",
    "pgfmathmin",
    "pgfmathMod",
    "pgfmathmod",
    "pgfmathmultiply",
    "pgfmathneg",
    "pgfmathnot",
    "pgfmathnotequal",
    "pgfmathnotgreater",
    "pgfmathnotless",
    "pgfmathoct",
    "pgfmathor",
    "pgfmathparse",
    "pgfmathpi",
    "pgfmathpostparse",
    "pgfmathpow",
    "pgfmathprintnumber",
    "pgfmathprintnumberto",
    "pgfmathqparse",
    "pgfmathrad",
    "pgfmathrand",
    "pgfmathrandom",
    "pgfmathrandominteger",
    "pgfmathrandomitem",
    "pgfmathreal",
    "pgfmathreciprocal",
    "pgfmathredeclarefunction",
    "pgfmathrnd",
    "pgfmathround",
    "pgfmathroundto",
    "pgfmathroundtozerofill",
    "pgfmathscalar",
    "pgfmathsec",
    "pgfmathsetbasenumberlength",
    "pgfmathsetcount",
    "pgfmathsetcounter",
    "pgfmathsetlength",
    "pgfmathsetlengthmacro",
    "pgfmathsetmacro",
    "pgfmathsetseed",
    "pgfmathsign",
    "pgfmathsin",
    "pgfmathsinh",
    "pgfmathsqrt",
    "pgfmathsubtract",
    "pgfmathtan",
    "pgfmathtanh",
    "pgfmathtodigitlist",
    "pgfmathtrue",
    "pgfmathtruncatemacro",
    "pgfmathveclen",
    "pgfmathwidth",
    "pgfmatrix",
    "pgfmatrixbegincode",
    "pgfmatrixcurrentcolumn",
    "pgfmatrixcurrentrow",
    "pgfmatrixemptycode",
    "pgfmatrixendcode",
    "pgfmatrixendrow",
    "pgfmatrixnextcell",
    "pgfmetadecoratedcompleteddistance",
    "pgfmetadecoratedinputsegmentcompleteddistance",
    "pgfmetadecoratedinputsegmentremainingdistance",
    "pgfmetadecoratedpathlength",
    "pgfmetadecoratedremainingdistance",
    "pgfmultipartnode",
    "pgfnode",
    "pgfnodealias",
    "pgfnodepostsetupcode",
    "pgfnoderename",
    "pgfooclass",
    "pgfoogc",
    "pgfooget",
    "pgfoolet",
    "pgfoonew",
    "pgfooobj",
    "pgfooset",
    "pgfoosuper",
    "pgfoothis",
    "pgfoovalueof",
    "pgfpagescurrentpagewillbelogicalpage",
    "pgfpagesdeclarelayout",
    "pgfpageslogicalpageoptions",
    "pgfpagesphysicalpageoptions",
    "pgfpagesshipoutlogicalpage",
    "pgfpagesuselayout",
    "pgfparserdef",
    "pgfparserparse",
    "pgfparserswitch",
    "pgfpatharc",
    "pgfpatharcaxes",
    "pgfpatharcto",
    "pgfpatharctomaxstepsize",
    "pgfpatharctoprecomputed",
    "pgfpathcircle",
    "pgfpathclose",
    "pgfpathcosine",
    "pgfpathcurvebetweentime",
    "pgfpathcurvebetweentimecontinue",
    "pgfpathcurveto",
    "pgfpathellipse",
    "pgfpathgrid",
    "pgfpathlineto",
    "pgfpathmoveto",
    "pgfpathparabola",
    "pgfpathqcircle",
    "pgfpathqcurveto",
    "pgfpathqlineto",
    "pgfpathqmoveto",
    "pgfpathquadraticcurveto",
    "pgfpathrectangle",
    "pgfpathrectanglecorners",
    "pgfpathsine",
    "pgfpathsvg",
    "pgfplotbarwidth",
    "pgfplotfunction",
    "pgfplotgnuplot",
    "pgfplothandlerclosedcurve",
    "pgfplothandlerconstantlineto",
    "pgfplothandlerconstantlinetomarkmid",
    "pgfplothandlerconstantlinetomarkright",
    "pgfplothandlercurveto",
    "pgfplothandlerdiscard",
    "pgfplothandlergapcycle",
    "pgfplothandlergaplineto",
    "pgfplothandlerjumpmarkleft",
    "pgfplothandlerjumpmarkmid",
    "pgfplothandlerjumpmarkright",
    "pgfplothandlerlineto",
    "pgfplothandlermark",
    "pgfplothandlermarklisted",
    "pgfplothandlerpolarcomb",
    "pgfplothandlerpolygon",
    "pgfplothandlerrecord",
    "pgfplothandlerxbar",
    "pgfplothandlerxbarinterval",
    "pgfplothandlerxcomb",
    "pgfplothandlerybar",
    "pgfplothandlerybarinterval",
    "pgfplothandlerycomb",
    "pgfplotmarksize",
    "pgfplotstreamend",
    "pgfplotstreamnewdataset",
    "pgfplotstreampoint",
    "pgfplotstreampointoutlier",
    "pgfplotstreampointundefined",
    "pgfplotstreamspecial",
    "pgfplotstreamstart",
    "pgfplotxyfile",
    "pgfplotxyzfile",
    "pgfplotxzerolevelstreamconstant",
    "pgfplotyzerolevelstreamconstant",
    "pgfpoint",
    "pgfpointadd",
    "pgfpointanchor",
    "pgfpointarcaxesattime",
    "pgfpointborderellipse",
    "pgfpointborderrectangle",
    "pgfpointcurveattime",
    "pgfpointcurvilinearbezierorthogonal",
    "pgfpointcurvilinearbezierpolar",
    "pgfpointcylindrical",
    "pgfpointdecoratedinputsegmentlast",
    "pgfpointdecoratedpathfirst",
    "pgfpointdecoratedpathlast",
    "pgfpointdecorationpathlast",
    "pgfpointdiff",
    "pgfpointintersectionofcircles",
    "pgfpointintersectionoflines",
    "pgfpointintersectionsolution",
    "pgfpointlineatdistance",
    "pgfpointlineattime",
    "pgfpointmetadecoratedpathfirst",
    "pgfpointmetadecoratedpathlast",
    "pgfpointnormalised",
    "pgfpointorigin",
    "pgfpointpolar",
    "pgfpointpolarxy",
    "pgfpointscale",
    "pgfpointshapeborder",
    "pgfpointspherical",
    "pgfpointtransformednonlinear",
    "pgfpointxy",
    "pgfpointxyz",
    "pgfpositionnodelater",
    "pgfpositionnodelaterbox",
    "pgfpositionnodelatermaxx",
    "pgfpositionnodelatermaxy",
    "pgfpositionnodelaterminx",
    "pgfpositionnodelaterminy",
    "pgfpositionnodelatername",
    "pgfpositionnodelaterpath",
    "pgfpositionnodenow",
    "pgfprofileend",
    "pgfprofileifisrunning",
    "pgfprofilenew",
    "pgfprofilenewforcommand",
    "pgfprofilenewforcommandpattern",
    "pgfprofilenewforenvironment",
    "pgfprofilepostprocess",
    "pgfprofilesetrel",
    "pgfprofileshowinvocationsexpandedfor",
    "pgfprofileshowinvocationsfor",
    "pgfprofilestart",
    "pgfqbox",
    "pgfqboxsynced",
    "pgfqkeys",
    "pgfqkeysactivatefamiliesandfilteroptions",
    "pgfqkeysactivatesinglefamilyandfilteroptions",
    "pgfqkeysalso",
    "pgfqkeysfiltered",
    "pgfqpoint",
    "pgfqpointscale",
    "pgfqpointxy",
    "pgfqpointxyz",
    "pgfrealjobname",
    "pgfresetboundingbox",
    "pgfsetadditionalshadetransform",
    "pgfsetarrows",
    "pgfsetarrowsend",
    "pgfsetarrowsstart",
    "pgfsetbaseline",
    "pgfsetbaselinepointlater",
    "pgfsetbaselinepointnow",
    "pgfsetbeveljoin",
    "pgfsetblendmode",
    "pgfsetbuttcap",
    "pgfsetcolor",
    "pgfsetcornersarced",
    "pgfsetcurvilinearbeziercurve",
    "pgfsetdash",
    "pgfsetdecorationsegmenttransformation",
    "pgfseteorule",
    "pgfsetfading",
    "pgfsetfadingforcurrentpath",
    "pgfsetfadingforcurrentpathstroked",
    "pgfsetfillcolor",
    "pgfsetfillopacity",
    "pgfsetfillpattern",
    "pgfsetinnerlinewidth",
    "pgfsetinnerstrokecolor",
    "pgfsetlayers",
    "pgfsetlinetofirstplotpoint",
    "pgfsetlinewidth",
    "pgfsetmatrixcolumnsep",
    "pgfsetmatrixrowsep",
    "pgfsetmiterjoin",
    "pgfsetmiterlimit",
    "pgfsetmovetofirstplotpoint",
    "pgfsetnonzerorule",
    "pgfsetplotmarkphase",
    "pgfsetplotmarkrepeat",
    "pgfsetplotmarksize",
    "pgfsetplottension",
    "pgfsetrectcap",
    "pgfsetroundcap",
    "pgfsetroundjoin",
    "pgfsetshortenend",
    "pgfsetshortenstart",
    "pgfsetstrokecolor",
    "pgfsetstrokeopacity",
    "pgfsettransform",
    "pgfsettransformentries",
    "pgfsettransformnonlinearflatness",
    "pgfsetxvec",
    "pgfsetyvec",
    "pgfsetzvec",
    "pgfshadecolortorgb",
    "pgfshadepath",
    "pgfsysdriver",
    "pgftext",
    "pgftransformarcaxesattime",
    "pgftransformarrow",
    "pgftransformationadjustments",
    "pgftransformcm",
    "pgftransformcurveattime",
    "pgftransforminvert",
    "pgftransformlineattime",
    "pgftransformnonlinear",
    "pgftransformreset",
    "pgftransformresetnontranslations",
    "pgftransformrotate",
    "pgftransformscale",
    "pgftransformshift",
    "pgftransformtriangle",
    "pgftransformxscale",
    "pgftransformxshift",
    "pgftransformxslant",
    "pgftransformyscale",
    "pgftransformyshift",
    "pgftransformyslant",
    "pgfuseimage",
    "pgfusepath",
    "pgfusepathqclip",
    "pgfusepathqfill",
    "pgfusepathqfillstroke",
    "pgfusepathqstroke",
    "pgfuseplotmark",
    "pgfuseshading",
    "pgfverticaltransformationadjustment",
    "pgfwarning",
    "pic",
    "rule",
    "savedanchor",
    "saveddimen",
    "savedmacro",
    "scoped",
    "shade",
    "shadedraw",
    "spy",
    "state",
    "symbol",
    "tikz",
    "tikzaliascoordinatesystem",
    "tikzappendtofigurename",
    "tikzdeclarecoordinatesystem",
    "tikzdvdeclarestylesheetcolorseries",
    "tikzexternaldisable",
    "tikzexternalenable",
    "tikzexternalfiledependsonfile",
    "tikzexternalize",
    "tikzexternalrealjob",
    "tikzfading",
    "tikzgraphforeachcolorednode",
    "tikzgraphnodefullname",
    "tikzgraphnodename",
    "tikzgraphnodepath",
    "tikzgraphnodetext",
    "tikzgraphpreparecolor",
    "tikzgraphsset",
    "tikzifexternalizing",
    "tikzifexternalizingnext",
    "tikzinputsegmentfirst",
    "tikzinputsegmentlast",
    "tikzinputsegmentsupporta",
    "tikzinputsegmentsupportb",
    "tikzlastnode",
    "tikzmath",
    "tikzpicturedependsonfile",
    "tikzset",
    "tikzsetexternalprefix",
    "tikzsetfigurename",
    "tikzsetnextfilename",
    "useasboundingbox",
    "usegdlibrary",
    "usepgflibrary",
    "usetikzlibrary",
    "value",
    "x",
    "y",
];
