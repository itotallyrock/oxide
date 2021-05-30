use std::hash::Hasher;

#[derive(Copy, Clone, Debug)]
pub struct OxideZobristHasher(pub(crate) u64);

impl Hasher for OxideZobristHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!("OxideZobristHasher only supports write_u64");
    }

    fn write_u64(&mut self, i: u64) {
        *self = Self(self.0 ^ i);
    }
}

use crate::game::OxideSquare::*;
use interface::game::{Square, CastleRights};
use crate::game::{OxideSquare, OxideSidedPiece, OxideCastleRights};

// TODO: Potentially replace with https://crates.io/crates/const-random
// The base zobrist hash for an default board with no toggled attributes
pub const BASE_KEY: u64 = 0xC8DEC4357946C4CB;
// Used for black to move
pub const SIDE_KEY: u64 = 0xA92CCEB891EA45C2;
// One of the 16 possible en-passant squares A3-H3, A6-H6
const EN_PASSANT_KEYS: [u64; 16] = [
    0xCC5EEF113797E347, 0xAA90BC6F508FC0AE, 0x735DA197A644D75E, 0x37744D11E638E6DA, 0x197A767F2768F84, 0x2051D4EE0123676B, 0x2B9AD8C00CFFB700, 0x9C54065D4D23E231,
    0x3D93FE652786B4DF, 0x946EEEB81F3B174D, 0x4C2EC39CEE8B9A0A, 0x276F22C3BA40D7E9, 0x252997C69EB74C9C, 0xEC926AE550EE73E0, 0x3CE2A3E88BC56598, 0xB6CF5D2A80FBEBD7,
];
const CASTLE_KEYS: [u64; 4] = [
    // White King
    0x84993F262ABB0E4A,
    // Black King
    0x49A617EA01D9B291,
    // White Queen
    0xA0ACB86F0695F023,
    // Black Queen
    0xE111D8788EEFCFDE,
];
// Keys by square then by piece for piece/square positional hashing
const PIECE_KEYS: [[u64; 12]; 64] = [
    [0xBCBD2C2F7DABFCBE, 0x875617FC113F9090, 0x314A7DFE25D739E3, 0x47F56D3649FEFA55, 0x2276E9C26AD34276, 0x776FE86869DACEAD, 0x4CDA34E6051BA0AC, 0x25800E89C0663865, 0x5634EEDA8F6E658B, 0x6947845BFD63D7F7, 0xE85B94FA980812E5, 0x1F8D1CF9944FD778],
    [0x63E82D59AE668F3A, 0xFF8294191FD12797, 0xD3AF0509C345130E, 0x901849841E223B79, 0x721D2AE551AC50E0, 0xE1D0FC8B991146B4, 0x578F1785DA5A5360, 0x67E73D3A61B0801A, 0xA398501928068508, 0x712B1876FFE58BEE, 0x8B878C1E93003BC1, 0xA7C3066360F208F7],
    [0x227A1A100D31A3FF, 0x6861CFA97E224B21, 0x8FA31F03AC0630D,  0x8F99842E1A4310F,  0x760639E8B269F8D,  0x5B5A4456033E62F5, 0x104592EA1780E7E9, 0x58E76D95AD43433E, 0x989BE44CC2AB1362, 0x5EA5B4AB668B1AF1, 0x877B9718C1389801, 0xB94DBA7F5E20B729],
    [0xEECC6ABCACD83CF3, 0x888814C0CE90BFA7, 0x5A46EB53DB79AACD, 0xB53BA4CBC4B9F2D2, 0x903B33F6FA0BBFE2, 0xB0EFF01BC78F12D3, 0x75A0158C798B7238, 0x85BE5E22FFB65A5,  0x83D7FCBEAAC09CC3, 0x1AAA5FFE8B5747C3, 0xCC02DCCEB253F09,  0x97BDCB4641D43291],
    [0xB6CC2E1A5F3BCE32, 0x9FBFDCE6881FDB26, 0x1F64C68479CC21AD, 0xFAB6AA7832F95158, 0xBBCF3EEA8911CC7E, 0x73D76DBD60291258, 0x32A85E233EF857AE, 0x9D72C7390502D7AC, 0x6F1739C34B67B344, 0xDF969D84A61A57F1, 0x6C6727DFAE596F6F, 0x33B1C2775CDBF746],
    [0x8F2DFF565239F7E9, 0x8694066FA7C57A33, 0x6E491ABA0ADD400E, 0x23BDB00CD3F36FF4, 0x1514F4994C36AFCA, 0xF17B6E986095EF04, 0xCC25705FDD219642, 0x62231637C9B014FD, 0x17FCE4F9A285980B, 0x2C44A0AB2B3DC6BC, 0x9C5243E05375A93F, 0xD28A18755B0B45C9],
    [0x376E7CB8F5210466, 0xEF7822C5C5C57251, 0x30EBA2ED5197F1E0, 0xE9DA2A851B3AFD17, 0xBBF190E08C18A139, 0x67E230BE30A1E31A, 0x86B67A41102B53BD, 0x240F02B0206484A,  0xD51E93EFE13150CC, 0x46D93E2E9A4D67B3, 0x7D2C9F1D5CB97808, 0x7E5DDCDD0C6D1E81],
    [0x875D8C04BB95FE81, 0x59E969E58561F1D4, 0x6A49C35BDAF37FFB, 0xE725C9DE0D3DC949, 0x3FDF4C33F73C58BF, 0x44637662190801ED, 0xF628079F241C485,  0x5C138BA66221CB0,  0xF4DF41419A33F4D9, 0x6C1E71E6D3186688, 0x2AE8A5DBD2D7E72E, 0x7B1B4F05DA7AC4F],
    [0x54B04DEA70B83260, 0x4A14E48624E4B1D3, 0x238E20FCA9063B3B, 0x69C7BE0CFBD9A4EB, 0x5BEB3B9CA51DBA3F, 0xDC3DB15E515CA7B8, 0x90F252658F1C998D, 0x487D1ADB67D7765B, 0x82286F7BC453DABC, 0x908CF828C4827A85, 0xE2B0FD2E530FF5F4, 0xAB584F2B9EC91B1B],
    [0xB6462F20FE42A8C1, 0x4CF8AB645B1EBC8D, 0xE9410F0EA35F01F8, 0xA5C18F5A3416B6B7, 0x1AF2394C1E318DF4, 0x1B72BFCD43F50672, 0x7EE0BDAA8EB1D943, 0x46946AC1F54E8A3D, 0x2999BE754AEC2106, 0x9169DAD7727F9353, 0x3CAE03250C891A9F, 0x233CCFE4AEE607DC],
    [0xC95BF4446F1113BB, 0x41868A63B923B9D8, 0x25C3E1C733ADBA90, 0xCF5A89C58391100,  0x626A394E6F50937E, 0xC225A9500E5C41A8, 0x7014FB42774D7625, 0x1DFF3BFD08C3DC62, 0x9CABAF2E3D29C6B1, 0xCBC29465408387AA, 0x14F59282CF4F79F3, 0x4E01E81CAEF6D7D9],
    [0xACAE411ABFCFE03,  0x6606060B1C2497C1, 0x5B69B0ACEBD43F0A, 0xCF261D70CA4A0DA5, 0xBC1CA0223DCBE770, 0x3D6473BB40F70284, 0x4CAEC1A74F7AE101, 0xD0980C5B21DB5C65, 0x658829197777BBF1, 0xF5FC7798C7A12A85, 0x9C90BE849F895139, 0xA5CF3ABDD0852F7F],
    [0xCE94767C46F49493, 0x54AB7A923785E18D, 0xA6690F02456613D,  0xBE01954AB856BD10, 0x5662D4D7322C10FC, 0xBED135CAF11B2BF,  0xF9EA2691AB2179F1, 0xEA162C63FA903D09, 0x35FC7815B102C450, 0xB094CDCFB6273E93, 0x7FBACB0DBDA50F22, 0x29F15721CAD66B98],
    [0xA6C8036491B5ED7B, 0xBB36485FE1E8E3F6, 0xF4A488E24571BE79, 0x18E9B92E1E98A598, 0xF42B6E3D9B58D82B, 0x1D933E4C6EC99FE3, 0xEC5907A84E073A9B, 0xC4F8717A2FEA61A9, 0xF2F35A563DDE9F45, 0x28C5E9B176AD0627, 0xDF12771992EC1246, 0x3AA4E5D9E5810670],
    [0x41D1D69A7EB74078, 0xFE4A37FB4CAB3560, 0x5F055BBF696787CA, 0x1E66CA0E48FA991,  0x83F505C7B109B558, 0xE7707B2B46B4A6DB, 0x572C58573D470CEA, 0x5989B37D75E920AF, 0x65D8CCEB73649B51, 0xA2BC8290D775A331, 0x659FA1E3CDE3B250, 0xAD80EF9D9324DC33],
    [0x6E181F30AF26DF11, 0x740DE1F008564A09, 0x9A52DEF82DE4800E, 0xD602600B84B96526, 0x6AFB479EAF5C9F24, 0x5C11BA263F8ABB5A, 0x8265CA263800A341, 0xAFE2E41E93070218, 0x70102A9F07E48FA4, 0x3B9F024D82B1504C, 0x50D3E1B5912B9061, 0xA645200E06DB27A],
    [0x7E17D334A661F5EC, 0x16F0477A6FDE45FE, 0x6616DBB4D84F9C0A, 0xC28B41D45FCA5C88, 0xA562DED7B5435FC9, 0x5625B1426858F0B1, 0x88E0079BBDAB5DAC, 0x125B95231E840685, 0x5A1911E160AB8469, 0x42D517E521C31BCF, 0x977E75D83BB4D89,  0x1986A8BF8656281B],
    [0x34CFE1DBB6C236B4, 0xC8748C60099BCF7A, 0xF7E1A6B17FD57F61, 0x6AF318ED405492F6, 0xB5114640CD060FBA, 0x9C6D60AFBB06B84B, 0x8256E205E135D5F2, 0xE3484662F424E4AA, 0x145BB338E633D6A6, 0x91BB96EDBE9336C3, 0x5F59C93E86069DC,  0x1EAF7C4EFB58FB54],
    [0x2E59790572672463, 0xD760791063D6B119, 0x1554AB5B18F4A1B,  0xEA91F7E13ABAAD1F, 0x82F2F01931AB599D, 0xC9CE496BE9CCE5A1, 0xCDE53EB2C5CD0310, 0x67150AACE03F17,   0x81C4DB418899F7D6, 0xBA5EE5C75E101CD1, 0x3C4C547EE278D196, 0xE0587058C0A8BA76],
    [0x2E3880612120DC4C, 0x6C2304C812BFC05,  0xA10885665D535BA3, 0xB12A3BE33B764C04, 0x134639DD81FF56C1, 0x8ADF7DE2316CB82C, 0xA85A61A495757D75, 0xBF7A38111B74C60,  0xEA1CF90508667BD4, 0x2E22A6BD3E5D7447, 0xFA209918A4D07A0E, 0xDAFA7BCA2987D49],
    [0x52B03DF59163F7BC, 0x58AFE80CCB66165,  0xCB47F9BB89C04A28, 0x72A18D89A6C1DF14, 0xA49164C775F17A4E, 0xEC0CF520C945483C, 0xF1018CD36D416FD,  0xCE0376C0906FE2DE, 0xF9B08CE888A797BA, 0xFFC64519B3B4BD4D, 0x286AB0A0A8A78A54, 0x86615029F5BD0B12],
    [0x6E6F33B380F9BADD, 0xC9AE6B05AA1AE10A, 0x49A802355C11589F, 0x61C05FA2E8A67086, 0x2187673712C2D996, 0xA71FB6713031131C, 0x2A4BA1F7F5246A3B, 0x5F5EF8AE50DE0C2C, 0x482FAAD0642B702A, 0x472F9AFEBF41EA3D, 0xCC2DE305D44C11A7, 0xD32F608A9DA24B51],
    [0xB2F4753E2C8B075B, 0x5C07ACE80BE7786C, 0x48781B3A40844D2E, 0x4D384A30660DAD80, 0xB8176B02674546C5, 0xCDAEB3E113C2E36D, 0x36F2CB8B9EF84873, 0x218A5F8923CF32E,  0x5239D4EC81631654, 0xA3D64CCA8F1B330,  0xF39A12F8AD701F4F, 0x92CF215D9BD0FF66],
    [0x806586843C10916C, 0xBE4569D0328D4C99, 0x2080FA791874A042, 0x9CDA2665110C62CD, 0x83F8B50F815834F3, 0xE8A7552D25982CA3, 0x5015662C8B95BA30, 0xABD2FB71E88CB231, 0xE9E33DD7C9C776E,  0x458E46D3BD9C8F06, 0x7CAFFBC6BB730740, 0xCF62C0D36077B3C5],
    [0x523AF458205DB0C3, 0x96C7751813249FB5, 0xA9F183A0B67D4FE0, 0x16FBEE7286CA5EA6, 0xB92BDFB907F3CE3A, 0xA1129D395FE8665D, 0xB2D3F65574A96BA,  0xEFC845A096C30D92, 0xFC4BC21C7ABF53B5, 0xB17B19105881FEEB, 0xDA67CF1D1E5F3452, 0xF9562F4B15A39114],
    [0x7C567D5F19E3E79F, 0xD6395FE910584064, 0xF780E698FD29A6C0, 0x14A4A20E276A65C0, 0xB66D46340547AA99, 0xCAF74A80C4D59AFC, 0x5442ADB5C57E45B7, 0xDCDC971BC05A9CE,  0x88CE1F9DA0246F23, 0xF5BA8DD4BE2B6E4A, 0xBD548B36DC35B932, 0x679BEFC498D00552],
    [0xEB0EC16FAA13402F, 0x44FEAC667E4ED6D8, 0x26312BAA0AA0069E, 0xCB8F12B379AADB29, 0xF797984C9058280E, 0xAD6370BA86E941E9, 0xD1310EC1D930507A, 0x1DB3A5A84E1348F9, 0xD1A018290F41C4CE, 0xC7AEF08855AA3294, 0xD9BD3B5046D24BE9, 0x678EEBFB08883333],
    [0x390A03D5BC93D102, 0x41D061DB2E277408, 0x380592E5FCB7A829, 0x9771D70B5A28DA50, 0x955EF1F46EB301D0, 0x2674677FCAE8D0AB, 0x2D4F8BCA3E1B1C6D, 0xC85FCE8AE7625CF3, 0x4A009C4A9573C3F3, 0xDF6ED7719FBFFECC, 0xEA7971A003593238, 0x88FB1E06E64DF966],
    [0x14B6AD74A1BE6FD8, 0x325AF886FB5E8EAA, 0xE88CE416DC2F90BE, 0xF6907D1878DAC859, 0xC299ECDA44B45620, 0x27D70608556E0D54, 0xF24F22FFAD58853C, 0xEA84C586D601F5C0, 0xBD2C883F12285AEA, 0x4BE80C6DDC7CBC82, 0x875EBFC238C7A9E3, 0xD809DE1B3531D667],
    [0xED047201D2C90C7E, 0x938478957AFD9A31, 0xF0345F9114EAEEAE, 0xAAD3EC827C7C58E,  0xCF40363F52FD5D36, 0x664C8C56019F6A5E, 0x73F0E4E955E57E8F, 0x6A196634884C58F2, 0xB2A9137F29286040, 0xD854485EDBD7838,  0x36BD2FC2BF56283F, 0xF74EAAFDF30FF098],
    [0xCE42B02E0184248A, 0xECFA4F13FF06EC74, 0xA7D12C2F92DCCE67, 0xB2F527BB58659F9C, 0xE51B6ECAADC2B71B, 0xA954CAEDC0E00A87, 0x983C86D449879ED7, 0xE6A1E9B35595BEE8, 0x62B1084CF6843C2F, 0x7CE6D096BE39FE97, 0xC2E991E9A5778BCC, 0x671213C15333557E],
    [0xCEE7D8B413191C9C, 0x4192220FBE7D653E, 0x5B60B4831F558E8,  0xCA1D6D63E6BFA363, 0x93BE14C8EC83DE40, 0x583AC4D44E26A599, 0x512D8219B1555967, 0xA7809EF24AA60A5F, 0x18A11322AF277C22, 0xD156A4CBB7057B6A, 0x4CBC6A55E03A50C,  0x6A04602EA6DFA31],
    [0x78346D760A10BC4E, 0x3895E347EBB40051, 0x95A552149BE313B1, 0x591496B93C3045D3, 0x1DA809F1834D8AE,  0x23932F6820457419, 0xD6121C28EFDDCECE, 0x53304B5497911AB0, 0x9D7CCEB7AC2CEE2E, 0x769EB8F9A2E7E6DF, 0x76A801DD90DAE5C2, 0x79C2BDA017D25D01],
    [0xC31D87E2B43EBCAB, 0x4A4B22BADED03607, 0xC05BE87F228950AB, 0xBF31B18C6102D07E, 0x6E7AE04F8928263B, 0xD54C5EE4E67E8FC0, 0x8DDA288DF4D70602, 0x3A5EB0C140A20CCF, 0xA81F54D7753E6039, 0x7DA40C268A6320C7, 0x43C4094739E77401, 0x8A37A80F44FBAD2F],
    [0x5D86996DCAB93567, 0xDD589D0224C21CDE, 0xFBA2FEE20984DF4D, 0x1300EEBEF87F53DC, 0x4004B8936021123F, 0x230E5D310794F9A7, 0xACD2BF83B9E82824, 0xAC6CF35E80AC6777, 0x4222D94D8523CEDD, 0x3C6D8BC0D824A470, 0x94F4B4CFDD1173D4, 0x3684556E5D659741],
    [0x323E36EC4020AA5A, 0x11B0A2422727EE94, 0x3A5CEFA1C0E14107, 0x10A0AACFAAC1E480, 0x85DB245D53236C19, 0x864EDFFFCDDC02BE, 0x62999802342CE505, 0xD68EFE4CFBAD0D4D, 0xB4A560533C059643, 0x83CCC85BA03E2F50, 0x4CF2A3FF1CC702B1, 0xF284C374E152FA7F],
    [0xCCF536B9BA8F3875, 0x30C198F0E1AC0037, 0x508B26CA94DD9F6C, 0xF0FCB6D4B756550,  0x7230DA6802731234, 0x914A977E1E89A3B9, 0x3064F9334DBA5FEA, 0xCCA2D477C271892D, 0x75884CCF07B2D822, 0xC4CA1406A035D5ED, 0xB7443CF47876D98,  0x6464F6FA0F7BE46A],
    [0xAD96BA6D3C4EB7C0, 0x338F07A68AB829CF, 0x31DE32D9D2F1F8AF, 0x3455FCE95157184F, 0xABFB8C419D44127B, 0xCE65235BE7B6C7B4, 0x3DD38B3AF0718F2D, 0x7BCEC4BA6660AE8C, 0xC8A95ADF1F8092D2, 0xBFD0EF84D73E97A3, 0x8A77419FC6DB8920, 0xF38E58DCC867A9C3],
    [0xF89F31F290478,    0x163610255CE81668, 0xEDF406F67488B74F, 0x48BC0B727001A743, 0x2BA3279FE28DBB4,  0x53D773A12D1135E5, 0xA4D319C9A10E2BD2, 0x116DF7D4C3CEF5B4, 0xB7F358DE059AF0FF, 0x57974A067BABA7F,  0x6DFB53E780605A43, 0xEE969CD21A247DBF],
    [0x37A4C3E506B0747,  0xF31FEEBEBF542D3A, 0xB9C4CA70A6DC0E07, 0x51E018EBA2990EC3, 0x1B938049BC25552C, 0xE3522E26165CB203, 0xF961E70254C2042A, 0x86F55EF8890B6576, 0x70E383D8899981AC, 0x6F7F32016215E827, 0x995521EDFD61C81E, 0x926A22F072FFBF6F],
    [0x14429708E51866B6, 0xC0F08A806F664097, 0x7E87B6B04B0CFFD3, 0x61C2CE4C3261E5C4, 0xB950F6F992F46DA0, 0xB1773170C97DB00D, 0xA36335D5BB916AF2, 0xA78AC0CF0EB5FE04, 0x65372331914E2443, 0xA84B5DAEC0A7917B, 0x44EF1028449433C1, 0x5633486F05C9DEB],
    [0xB6D0CB18C7B278FB, 0x6AEB6FCF905DF5A1, 0x7A7A31A2D8A8717F, 0x17E88FD5FD4D5733, 0xD1AFBDE4E6250E,   0xB9CAAAAD7EEC4194, 0x7A8104D208158879, 0x4AB3658D59164186, 0x9007CF0FA317375E, 0xC61C6A77EA39E39C, 0x3BF3C4D1F7F4BA08, 0xBB98B2EB6307C405],
    [0xA5896291BB42F549, 0xD0CBFF48CDCBC572, 0xA7EE204DFAD5BC6A, 0x9AC1A7EC1B9C5F1A, 0x8294F675E9577764, 0x1D671DE4D33F4F70, 0x66517572216726FB, 0x9FEC8FFFB076A76A, 0x15039581F5738DA8, 0xE1ECA97577974D32, 0xE4C5F6CB1B93AF6D, 0x4D129A707B0AC8EC],
    [0x6189396AF01F8495, 0x2EE8EDE218D65FB9, 0xA4F77E2D471539C9, 0x191FC93DE6D9AD16, 0x69D25DD52E693695, 0x2A53440B0EEC4935, 0x7026E2BFFDAFC546, 0xCB62B3A5808E791,  0xC7DF6CD660438A4B, 0x5A9F19B373AD7D49, 0x23E6B1A109DDFFA1, 0xDA556E2B8E65F704],
    [0xFD5C5E53F18DD3A6, 0xE15BBCD8100B11B1, 0xFF5C5F0E56CA0491, 0xA6071D3907A6A447, 0xA1EA457BBE8735E3, 0x386959B412646EB2, 0x5B17FC2124415ED2, 0xC068FE94FB717CA3, 0x9575ED44719DD229, 0x3667EB64FC533A85, 0xBAE5783641DB7334, 0xBB024D731DC24D0C],
    [0x1FDF3067BEA149C1, 0xDFCE291E2FE491A7, 0xF501C81888953C66, 0x5FA407EF7EACFC53, 0x527F872FBF8EEA62, 0xD8BDD1C137D1C36,  0xB7F3717E01D12C16, 0x1110ECB74D1DF3F1, 0xB0F7EDBD25547561, 0x17F5422BEB08E3F2, 0x28876E94E5EE892D, 0x198F58DC39AB9973],
    [0x42A99B486008A903, 0x46A52C5EEF273F4F, 0x7FEF872B4763B969, 0xCBA441156AEF1D7F, 0xC92B57494E6C2B65, 0xC953533326054C0F, 0xCE8B913684C90592, 0x146A611584AA05B4, 0x1B95687840B744F4, 0x91827031CAF1159E, 0x890D616AD4DA3428, 0x6ABD16651732022D],
    [0xCE4409585A3A4AF1, 0x380D311B73FE18E7, 0xBEFF988BFB329DCC, 0x9ED8E9E7A99C82F8, 0xE7BE2B87C553C114, 0x2AC9D1B0ADA8EC61, 0xF1F7A5688A8D9ED6, 0xF59B11DFCC42A5F,  0x415E9D1668142011, 0x21C4DA1F059FFB47, 0x9CC292EACDBCDE84, 0x14EB650BAAC9A27F],
    [0x6D58F6EAA5B9EEDC, 0xBEBC9E6C723645C3, 0xC430AF2DD2A102B8, 0xBD56A6766F1A13FB, 0x524AB2D1A3645815, 0x972E080BF0EA58D5, 0x380CE998EEF6C953, 0x55EFFF356AAA144A, 0xB2A5204718E7C9D3, 0x8D5A6B9CECB78D61, 0xAD4E876818E00384, 0x76D7FDF7F7A63137],
    [0x20F2DD285D853862, 0x7EEE812ACE880B78, 0x7F452607DF1D3895, 0xBAD8A1B377FFEA94, 0x273A62F53EECD97C, 0xBE2D9D5D68043C0,  0xAAEF99E0C586D735, 0x9261604C3863D6D6, 0xD705782EC05F86D1, 0xEB73621E967C7A81, 0x6264D27DE7FB8B35, 0x3A99646397C3A420],
    [0x17090289C9651196, 0x5F05572BAE72F03E, 0x86B17CB904D25277, 0x985ED20B2BAD68A8, 0x2C15E0DD60CAAE57, 0xF0DD94155C150492, 0x784D7B1B5DD3372C, 0x92639C50B8F8643D, 0x6FDF404F35FF0652, 0x9C5A66332D104C67, 0x9E6E618A6A854BE4, 0x2E9A88E3C297F808],
    [0xD653298F4CFC8335, 0xA3D97654902EEB13, 0x20588237EE5E7AA7, 0x451BC4808ED30A8C, 0xA3FD384F5CF215B7, 0x7CFA762E9F214E6,  0xB26FC261A7B1B896, 0x7D5AB840DBC5CDDA, 0x4E1EBF72D75557AA, 0x637E964AEE82DC0A, 0x31F4061D76F3C95E, 0xE5875613B74E7129],
    [0x1358488479D16766, 0x39FA8819BE20C9E5, 0xB958068D7604C5CF, 0x449B39AE8423E2E,  0x685D9FE1C5430A5,  0x3FE2F97EDE448C65, 0xF083A0B0C12080AD, 0x3A0E269289484BD7, 0x26E6E5AFDBD0E935, 0xC9008DFBB2F38391, 0xEB0F89713FB8F596, 0x5D7B33904D2F478D],
    [0xF010E350C1D23084, 0xB11542FFDB5B488D, 0xFA1BBB96AD885511, 0xD7001064A53E328,  0x6B6A5C53B375A3BA, 0xA0AAB8ACAF62B833, 0x9D65EB0A9708270C, 0x830EB3516664E837, 0x4066FE756CABA452, 0x59DA43F564BDC62D, 0xA93E04AE6F0BA79C, 0xBC78990852AC78C6],
    [0xDBB0709297A84DD0, 0xCA480BDCDDDAC43B, 0x5923E0D5BC2A615C, 0xF59C424197ECBF9B, 0x1CD3C8175637B31,  0xAD081B32E7CCDB8,  0xF4A2E6111C6C9E65, 0x9B0CF6E224027BCF, 0xAB2CCB16ACD55BCB, 0xEF68D690FBEAD92D, 0xBC5A6EC823166637, 0x180EFBC1AA48B07B],
    [0x723779BA8876CE58, 0x8D5ED10EF4EB58EB, 0xEE4B7AC38CA7CBC6, 0x71EC7244BA68FE2D, 0x94A90CD0BA0870C6, 0x7527B416AB3F6853, 0x158F0D15C73D1E4,  0xCD2A0ACFC17AC2B,  0x489830D3F6C0A185, 0xEC1EE3EF620D8EA1, 0x4E1C0FC288CA1D7A, 0xD13A6A3EFB2AF639],
    [0xF149A4CFE4B4752D, 0x69696C4F337D5E4D, 0xD72EA09A2A702537, 0x18CDF62D0F67AFBF, 0x45A652DA8B913E2C, 0xA0DB8B237CA41AC5, 0x1D5390EB2A60AECC, 0x32B0003EA95A5D33, 0xB229967C64747999, 0x812E3A672F216CA4, 0xF15FFC2B073B6BE3, 0x8ACC8A965963C08D],
    [0x39B1B5ACDC7BA46E, 0xEAE4AD67C17FFCEE, 0xCBF8D57638A687E6, 0xA25D2D7E284F8B70, 0x38FD4D903E9F90C9, 0x3083C4F242409C1D, 0x94DB7E90D919934C, 0x5DD7518C34D2FC8E, 0x4AA14F72D82FCCA9, 0x7E1A053218FEE895, 0xC83EA32B9C3AEB25, 0x90F9E63E74AAD712],
    [0x7354824E568324F1, 0xC2E718C3AF0D2CD7, 0x269062605662E95C, 0x9CFB4CA61E256E84, 0x9AD359DEF2A8A96D, 0xB8F3938620A93C42, 0xE2B7DF8101CBB76F, 0x1FC6DD4CEF2B520D, 0xE12337D79D74E9FE, 0xC759B24F504AA46F, 0xFF5278C08C3080F6, 0xF5C13EE8E3220BEF],
    [0xAAB55DFC683F07A0, 0xBB569B79CA7CB8A0, 0xB439FFC4E11EFDBC, 0xD34DDB89AD3ACBCE, 0x5F3545C1847D6076, 0xFBB7ECF99B6D2207, 0x9F3B39C0801F8822, 0xD9C5B7BA823E6224, 0x6A29FDC09E58ADC1, 0xF78746E12C71B173, 0xB601C5D3D3D1EA3B, 0x41E35433E9D6C06C],
    [0x45499F5B7C4BD910, 0x5BD38607851E4382, 0xB1212FF735A22AD5, 0x8C29187CDE779BFF, 0x87100C21823556D5, 0xF20B591CE223B985, 0x7B76EB6EB2A42BFF, 0x45375884E6AE87C5, 0x6F5447073B2274ED, 0xFDD4387BC3540BC2, 0x20D7D0AD866F5D45, 0x666DB2E3263D5B33],
    [0x94E02142A277918C, 0x9E121E5903DF80A9, 0x33F552A696A6D0BA, 0xC91E23B4BE4C6249, 0x16652D46A6AC34D5, 0xD2E969A11F32D406, 0xF8DBE1B707F6B273, 0x2187DD7B4005D9BB, 0x90B700E51DC3B6D1, 0x950DBDB3FF942002, 0x29B9F48C343C4A73, 0x2487AAA27DA1CC21],
    [0xB8F6F70EBDBC5630, 0x9D5409AFDF3A7D06, 0x3BBF4E7D5198773E, 0x330CFB767521E96F, 0xB3F18DA1EC7C8992, 0x21886C6632DE4BDB, 0xFD5EB5AA3EEC05B9, 0xE152DCCA823B842B, 0x5B90585C75E7639B, 0x37C1C6D077F16BAE, 0xFE6CAACE4255DEA5, 0x13305AF78CA69D7A],
    [0x46A5328070592955, 0x1E87ECCBBD86209E, 0x573560951B7B39DC, 0xF266286D9FDF94F4, 0x3A54202EB691F2B8, 0xF6786A6B966BC518, 0x25E3B72CBA434484, 0x4B8FDA0911FCBE6,  0x3DFAED88E0F683C5, 0xDE469EB5D8A664ED, 0x51E5841261426823, 0x29DBD31C802A5F77],
];
#[inline]
pub fn en_passant_key(en_passant_square: OxideSquare) -> u64 {
    let index = if en_passant_square <= H3 {
        debug_assert!(en_passant_square >= A3, "Illegal en-passant square while hashing");
        (H3 >> en_passant_square.offset()).offset()
    } else {
        debug_assert!(en_passant_square >= A6, "Illegal en-passant square while hashing");
        (H6 >> en_passant_square.offset()).offset()
    };

    EN_PASSANT_KEYS[index as usize]
}
#[inline]
pub fn piece_key(sided_piece: OxideSidedPiece, square: OxideSquare) -> u64 {
    PIECE_KEYS[square.offset() as usize][sided_piece as usize]
}

#[inline]
pub const fn castle_key(castle_rights: OxideCastleRights) -> u64 {
    match castle_rights {
        OxideCastleRights::WHITE_KING => CASTLE_KEYS[0],
        OxideCastleRights::WHITE_QUEEN => CASTLE_KEYS[2],
        OxideCastleRights::WHITE_ALL => CASTLE_KEYS[0] ^ CASTLE_KEYS[2],
        OxideCastleRights::BLACK_KING => CASTLE_KEYS[1],
        OxideCastleRights::BLACK_QUEEN => CASTLE_KEYS[3],
        OxideCastleRights::BLACK_ALL => CASTLE_KEYS[1] ^ CASTLE_KEYS[3],
        OxideCastleRights::ALL => CASTLE_KEYS[0] ^ CASTLE_KEYS[1] ^ CASTLE_KEYS[2] ^ CASTLE_KEYS[3],
        OxideCastleRights::NONE => 0,
        OxideCastleRights::WHITE_ALL_BLACK_KING => CASTLE_KEYS[0] ^ CASTLE_KEYS[2] ^ CASTLE_KEYS[1],
        OxideCastleRights::WHITE_ALL_BLACK_QUEEN => CASTLE_KEYS[0] ^ CASTLE_KEYS[2] ^ CASTLE_KEYS[3],
        OxideCastleRights::BLACK_ALL_WHITE_KING => CASTLE_KEYS[1] ^ CASTLE_KEYS[3] ^ CASTLE_KEYS[0],
        OxideCastleRights::BLACK_ALL_WHITE_QUEEN => CASTLE_KEYS[1] ^ CASTLE_KEYS[3] ^ CASTLE_KEYS[2],
        OxideCastleRights::BOTH_KINGS => CASTLE_KEYS[0] ^ CASTLE_KEYS[1],
        OxideCastleRights::BOTH_QUEENS => CASTLE_KEYS[2] ^ CASTLE_KEYS[3],
        OxideCastleRights::WHITE_KING_BLACK_QUEEN => CASTLE_KEYS[0] ^ CASTLE_KEYS[3],
        OxideCastleRights::WHITE_QUEEN_BLACK_KING => CASTLE_KEYS[2] ^ CASTLE_KEYS[1],
    }
}