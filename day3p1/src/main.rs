use std::collections::HashSet;

fn main() {
    let x = input_string();
    let backpacks = x.lines();
    let mut total: u32 = 0;
    for inventory in backpacks {
        let clean_inventory = inventory.trim();
        let half_length = clean_inventory.len() / 2;
        let (first_half, second_half) = clean_inventory.split_at(half_length);
        let first_half_chars: HashSet<char> = HashSet::from_iter(first_half.chars());
        let second_half_chars: HashSet<char> = HashSet::from_iter(second_half.chars());
        let common_type = first_half_chars.intersection(&second_half_chars).collect::<Vec<&char>>()[0];
        let mut priority = *common_type as u32;
        if priority >= 65 && priority <= 90 {
            priority -= 38;
        } else {
            priority -= 96;
        }
        total += priority;
    }
    println!("{}", total)
}

fn input_string() -> &'static str {
    return "NJvhJcQWTJWTNTFFMTqqGqfTmB
    VwVzPldRZVLVRmfsvfjvqfmm
    ZDPDHZHVcvDhbvnv
    FHHwHBzzVCWWmmCzCPrVmgBwbLTtRFFbbbttRGRLjTcLpbbT
    vhZZvdsNSdSMdNvjncppCLcLnGnj
    CDZZsNZMZqdNSdlNZCqrzPHDzgrgzwVVWwmwwm
    ndlndntsFJntFvccLjjLrjBShcBBfc
    GpCGHzVwmmzqQWSSSfWHBhQL
    mpCMGGCZVzVwGGVwmJsZnFtZnTSTJtdsvl
    nCnPDGmDNmVCsVQDmGSWqvzchWSjjcWGqS
    gTnBRLpfTRnrTdZgdLfRdrThvqcvWWhFFWvcFSSgjqqzjv
    pfZfTMwrbLTTfsbmQtlVtHHnbs
    wNdSdsbTvTZMTvTv
    rrdRWdWQhFVdHWBGWQmmmnnMvCfmnhvmCmtZ
    rJrVDRWpGddpbSlNSlspPP
    chTNrthMMwWMTjfsmRzZszJpwm
    BLnFFCngbcBnbbldDlpRjGpmsCzGsGsRGmmG
    dqvnvlgbqtcPPMhH
    QcLNqZbCzJDQBJJRpwzRpdnRldgnpf
    GmmmvVGsHrWffrlwdCWd
    CMsFVVFjCmFStGQbbLZNBbJBcTjc
    LQVggbQvcLbQLHgvVLhWGGsChssrMWfzGccc
    qDnRTTRqJttPfWMChJhGslWlzh
    qRTRwPBTBtRZdnjnqqqnQVbjbNLFbbfLgVmgHLQm
    cZbzwCwZPlJcMLrNSNfHWNBBNZ
    vsQsDCqtsDhmtjVrBNWNjBHrhr
    TtDTGnvTlgbbRCGg
    BgBlplHlsgNNsJlVpBtPwJhMPRRQSSttRtSP
    bvhTnmdFTzddStwStQRddt
    ZnZDLvnvqZzbbhFzzmTbnFsVjVlNgsCCNVsVLpNWVgsB
    TdptqrrcVGhhzFtw
    DRnSfwJlDmmDDVGv
    RCSQNSCQZndwbcMqQrBB
    wvRlrlwVwwqzgbZRdCJBWfmdzCWfBdhf
    cFcsQpNtLLsGTtNGpMdPmDdPBmmBvJPWvDtC
    TpjssTFFvLLLcFFQpwbwwHngjHRrZRqZVH
    mqqddrPPcPmqPDlrQnjTrbvMvbHzzsjjpTvz
    gtBWgGgVhLGWHzMDztzstDHj
    hfWRhBBNBGgLNQDPwdNPcPdw
    LhQzdhhbTzpMhddhhhTzhnZcBFllHZFtrrHZHMHFjlHr
    mwwssqDvjptrvplr
    NCSgVDPDwmDgVJVpLfTznQJdhfLhnhQQ
    GzjzDhjhhZzcrRgQCBjBPBBjQCgT
    vHHHmntsbSgLwbsSmNHbwNbvpqPCBVppCpFTpTPTBtqWBCqV
    NJbwNSwdndvmvwhGhgzcfMcDJfgJ
    GncgDvvcMGnttjDvrgRRFSZZLZFWdJFJwGQwZBWZ
    bPqpChPfsshfZZBdZdLTFZ
    lNqqsClmbsNlPbHqPsmblmsrHdvdMngcVrjggvrvggRDcn
    bDvtgVVVpMQvjQWmQL
    rwTflmlfZJBBdQWQWjQqdM
    HsJJmZZwscHrwTrcRbzpcbPgtCSbgz
    CsCsRvshMjpbqCqf
    ncblgDBgtDmmmTlBgwlgbHHqMFHLqPDMHPHHpqWfFM
    TcBctSmTZTtSTzsZvsvJZRsGVb
    znznvngttwltzlLwhtThHbqHPvNbNHSSHmmNWHjP
    FBcLrRMFQpPqpPSpqHHW
    fRQMJZJfrcMcMVrQJJftnwCzVCltgTnstTVnVL
    MfLlRfCMrLzRlQgwNqQFcsGd
    jtTjjBTvbdqcGjqFcj
    vvShDSBDppzhCmzq
    plWMptTvfrnncvcRfwqzqLGhzhzThNzNNJqD
    jSdSHFPQQbdPCQCssjSbBmhJGNZZNGNqqJNBlJqqLh
    VCCCVCQgjdddjCgljCjbbwgRRttgrpftfWrgvpwpnf
    MWlbBcPjjvvjPWWMPqgRQZfJZDGGbRZJffQQwh
    HrHrnncHpzrJQJfVDQVR
    zzsSTtSTLzsspSdtTmHHmpmtFgqcgPlgFqWBqqqBMdWWvFlg
    nSqBbJbqlnBBClVZcMgZVgcP
    FQwrwHrRwWWFBRPNgNgcCGZZZC
    rWFWFTwpwwWzHrnDbfJDLDbBBbbz
    BMmNtLMMtFCNFNMvvLmcndpgcdgppPrgrGPPrgJD
    WVWWhbTtVnGpjrrPhr
    HWssSTHWfRHRsQQFLvfvFFCLCNMNlt
    sTmDsQffVrrLCjTFltTFWL
    BnwwQBJbJndMMRzMwCLlWlLWWCWLLtRlWF
    cqqBMcMqwnznMGzcvDmQhrvssHmPDVssrP
    pQGQGJDDrDVJbbfVzvvgPcCZwhZhncscZWWc
    SqMMlBBljMmRlchhPTqThCZnPs
    FMjMBmjRNFHQJJpHVhVDhG
    tHNNdBdNtBBBMgsMpsZm
    wVPzVvbwqzhrVqvjqzzsZpDsZDsZmsCPCgZgCM
    bVbvLThvvbrWqHmmnJLdHdJQLn
    PzTspPZpdLLDZTplPLpPDpvbfhnqNvqzfvNMzQQfNwnQ
    GWRHmjmFWMMSnhbhHw
    JWWcmtBrBtWBFWGJpsgTgldhLVLpJl
    DwLMDzLMhvMcwvgdVqWWlCVgvlqF
    TTSBBRpbStHZVgjWFldjRVlV
    SnbTBdJBmnpQzMPDMcMznr
    nNlMNBPPNtJQnbZhZsgSbh
    czzCjcwTdvSbgQNcgNQq
    VTdNdGDTzDTdlFFPtBrtLtDr
    FMbbfMlzvFsmgVZmmg
    SrNTHGmdSQDqLhtQhhgggs
    dRDTSDPPcHRdHGDHlwJBbmwljmMcfjbW
    sQgWLtqLtWhdqlpNZRpG
    blTHTjlvTCJnJvRZdGGhHHGZhFGV
    CCDlJclnCmbrmBMgcwcLWtcBsB
    vqPWWvqwwCFvFZfZPRFRrcGQrQwsDrNcrwnbDNcQ
    LVgJLSBBVtzTLzBMmTMJmLnnDNQcrsGbsQbNbrbDjs
    zggVSmmhVdfqFhvHWG
    WwdndGGmmmLwwwmRwWSncLRnZqZqhqZthBtqtBqZBgtdtvMH
    FfHHzlQQDsFzzrNsVTfttZvTvttTqqtbqb
    lQjFDNQFPjCsVCCDjGCwwSGGnccwcHppGp
    mrjggcFsFMjdjZRpSZpn
    NCqfLCFNbQPzPPlPzNfSRTRZdSdWWwndpqRSSd
    vDvzzbPQFNCFtllLLNMBhMcDHGBGMggMmcBc
    jhjlBvvnjbtDNPjtSjBDBbDNgHggrQrhghRQrqRrZcRwwqVg
    pLdTMsWdLLmpMdqZZdPdVqZgHPwH
    WLTCGmMLfPSlbGjlnnJD
    gtbwhgHbHgqqbgQthgQLtZZCRjMcjjnRnrRNJmMRJrNhRc
    bGWVTTvDvfpVFFBpvvVTdRDMJcrccCrJnMRnNnNCcc
    FVWTBsdvdTzTBFWssVQtLgSQtHqqPzPbqHbw
    dlzrPTSSjSrllzWhsvVmVtTRTWtf
    bJMpLGcqGhNbJQttVQmmvRWWsp
    qLbMwqqbGHFGzrlZrjhPHCrj
    rNrrffVlqqrfLlPpltcBBTTGRzzZRPRsBTcJ
    msbsmWSsMmQwjdMbWMhMhQmcRZRzGjTBGTBcBJBjCHJGcC
    FwWbvdhbmrsFrfrgsN
    rHjrQHdhdQrvSddcHWLssBSVVpBSWWWWWf
    JNfTGtqDwVWBMBMpwM
    qlltZgfJFvcRgcRjvc
    CqfcwfDqwwmRnnqmRdNRBTRTRrdGdNpTvF
    WVbzsZszBbrsvpdMpdQM
    tJhbVZHWLLHDgnSwnSSgHB
    TZCqqlTsqpZVVsZQJSBSLpLmppnJzmFz
    brSgNtGjjRjRRjDddDtrRJcJJbJmmwcmBmnPcJFwFB
    jgdRtMjNNjfqlMvShvSZSZ
    dJTdqCwMNCgqTQllGBdlGBmmmZ
    fcVfVcnbVfrwDLWVfncZBQPlBHRGljLZQjHGQl
    brwnnfSFDvfzCTqFzgMJTh
    njnsPBjjsrrnGLnbTTjGvcldQPCMllNzMvRQPCdd
    ggZgfZtmZVpqZqZWDgFmgqfCcQRcRcWhQcccQddMcvRQdQ
    tfqgggVgHpDwDtfwbGLJRjbLjsrLTj
    JmrfrmTlDWTfgQCdHCdpqBvQdD
    jsZtVzNsSNVQQHnBlVQR
    PljljFjPljSsLPtFLTTgTcFrrfMJmrrmrr
    hmGcmmndhmGnfmtGnDzFLwrFJQsQFzNFrNJG
    ZSqPlSWcWlbgqWVTVWRVZPrjQqjzjFNJzLsNJsLJNqNL
    RHcWTZbSMMMPgZcWgSWPPbVMDnBffmtdpDBddfnnvmCdfC
    vSJvsbFfJfvqCsTHJswssJnLTZjjhzrrzLrzLMrzhdjM
    pBNQDPcpmWDcBNgMMnZPVjdddnndhH
    QWlDgmpmgDBlGRgDDgffSqwSwGCwHfvqwSFJ
    jvlgvMJclPdGdtdcjMVmMHbFHFVHWHbZHZ
    CwhLzLhzQpnqfpfqDVHCHbsbDFZDmHmj
    LnBzfQjSzQrPvJvdSSrr
    wpcvcsqclDCnVCVvWfnZ
    BLRMRtbnbbBLNCjNCjVVZhbC
    rFgMPSRnrRpmqpJwqFDs
    LZQNQbMrZppLNLQplvlGLNvVmmmfjbwVCfjbwJwCmBCwfj
    ShTPRFtTHZPCsnwswsFwCF
    WtHRPdThSqZTRtDqtdRWTdpGDLLzrNczvzMGLlQLGDDM
    hdcffBvldjhCMljqPwWwWNwWdwqHZr
    LtQmbQRVsZQZMZPQSN
    tmMRsJMpDhjJzJhv
    wNQCMFCDQDBmrHmmRWrrHN
    SShLnfqpcqpSZSfrzJvRVrvfrrJH
    cRpqdGclpScltTQQtsFQMQsTCT
    NCjggZmgfBgnBmgWbcwcTFctcWWfvb
    HsDGthRGrtppSQpbFFJTVcJdFbTRvd
    rPDGhDDrSzZLtzBLZMCB
    RsBBMBsCBlFFCgRsBJzlMjMPNSdPhSrSrzLbmSDrDNmDSd
    pZHZZJpGHHHpTTHvTncZqVLdqLbhLrDLdhrSLLbLDDdD
    tGtwnJccvCtCffMBgt
    wbddvVjfwPhbjjbDbbvbjvTNCNmfHZfpCZRJNzCmJmnJNC
    BslcLtclZWsZJWNrRRNRpRmR
    BSLBlScGtFMcssMBBFGLlQZTDZQjPddVwwbTdvvdhTZb
    NSZHzmLZBnzHmLLzLSntDttDDtddhDtttDWW
    QgfjsrrvNNJwtMddcvcvtq
    jrfgfQpQrTTVLSNBClFV
    GQWcWWPPQRcrJQNDdRcDmmLCFSnqNSmqhCNvFnql
    zHfwjzpMjwZmCLqvvnlljC
    ZgtVZBtHHZtgQGgPrbPRJdPv
    TWdWpJTJTdgLWfWLlLFLrfrgBGsNqhGslBGHqSNqqBNshnws
    ZpQmjzbZZCjZCCCPZtttRCCwsBnHNssBHbShsshHqsGBqN
    RDRRPpPCzmZCtRpVVJFrfTfWFLLJggJrDv
    pDDFlglsvFMgntlTMMqNffmTdfddRM
    jhGJLVCHQpHGQCCzLjWdTTdZZdNdcRWNccWfNN
    jQjSGjrjCQLhzVSLSCSHGDpngbrnDFtFBwBglBnBvg
    wsLzstsgszcpcGLHGpcgcghlDBvQvjQvbFbQCbJBtCCJJv
    mnSqRSSqSRThWRnmWWRSJDFTFCFCblbBCFQFCjFj
    rZRRWqSSdZZfMVnZLspPsMgHpzMhHGPg
    mwHrCLSWWwrsHCHDDsVrsmhfFZFnSSBlFlgZbbgBglbggj
    GJdpcRtGJvNRdcPtdpJJdbQZfjfQBlnQBjnBtbfFnB
    qcPpqqzFzJqvPVCCmWrVwhrWrz
    jjMbvbhDvnRjNRGMmjbMZftSSwwwthJSffStctcwqd
    lTQrVlpCVvCcfdcSJqLVcw
    srHFWCHrFlrHlrsBsprljjRmDZZnmbDngNBgbNZv
    MgTlQJlTQJZWpgLrRssrVqqqpRts
    bBNbbzSSjMBPjzhMjsPtRVVRVPRqLttGGs
    SjHBbfjNCDfjZgTlZdMJnDJW
    lpThgTwtplhghgwhThqnnrdZctSZSjSZcRSRfbdrrc
    RBVBGvmBmfdrcvrbbr
    PmVGNGmmGRLLQwwLqTnglQ
    nHwnBwBTnFHQwRsMhwghmzcm
    GtprdCpdtqWdbqbrfdnPPszsWmRzRnShPszS
    dGptbCfCrlnVDBJNLDLLVDLQ
    CZtCjhTndCzqbCNq
    dwpGvpsmwGslDszrNNrzqDMzWMgJ
    vmcGccvpBVPTVTjTdTTTdZ
    jWZhvZLjZfCZDwrDrSSzJGhVdJccscGsgV
    blMBlRqqqgSJLBLcsJ
    blmHLmFMMMnRqLmMMFqHmfPDfjQDnCDDQrZvfCjvDr
    rnvnHrDLFZmMFLvrHQBMGQggBztzglplRl
    sbWWhdNzsshsfhcsjJJPPbWdtQGVGllRTRjRRgBgQlpRlppB
    PPCCwNWhPhNfWCzbqmFnDFFnCDLSrvZS
    GChNjwWlWJWTJZBggvdgnQgdhdnd
    HPsHfHHrpHDpFFrcSfsfpCMmQdntLBMgtmtBgDdLLC
    SqpPscpPzpSWzjlCjjCGjl
    nvgLvcLgvgvngbLprpJNTDCCRNVJrNPlDDTV
    WZsMtsffGQtMzWFqFmWmWsVNJNlDwwCDVRTwJlCCDVLz
    BQfGZGmmsMWFstWFmfMsfBccdncbpbSbvbbvHnLbpc
    tsmDsvswNZmcZTccfh
    zCTpGCbWBRWFWHGRFZJbMbJfnrhnhfMnnZ
    TzFGFBRLdpHHNNQddDQDvwQN
    fhBBpJgdHddjZQfmVmNzNNLmFN
    qvMRrvlbwqlbTTMBMvLssFNmVzzwFDmLLzVD
    TRSRWqRRMcBHhGHcdGgPGp
    lSjHmtmnpHStblnpSlHSrtmMzLWzqzqCZDDTzTTWqMFqCqVV
    sLRLLfPPRQfCTqqVVqFT
    dNJgRPNQNsJJhBRvdJvQvNNsjSrrSmrcctpbpHtBrBjLjmSH
    nwFwpppjfwSlpLTsqsTgNshhjM
    ccBRGvtsmgGNPqNNGP
    BCcJHvssdcWBCVmVHSSrZrwVzblpwbzZnf
    rcfQRrBPPczjcRBctZDNlnVNHbgZGjVDjN
    TvMsFJGSFMhJnNZlwVVnDNTZ
    qhSqqmqLCLhFdJLqSvLhmQRQRWcRPczPtzrCrWGRBp
    JVhdPhsFPFqLDBHVdHLPvhHDCMwcgJJwbwRgnnCMbwGwcmGC
    fzjzpTZTQQQLwCbgGgbMmQcR
    jzNpTzfSZtfNSWZlVVtdFFFDHHqLHVqv
    TwSNnSnSGVTpNppGlPTlTcVqQrRhVBqdqBRqZqQZqQ
    DcDCMfDbCMHJdrRBqbdjRBRZ
    gvftMCJHcHfCDmDLgfMmMmmWlwWnWsTTwlGTlWTwppNlGL
    pbGMbllDQPhhWWQDpPgVGlMCvRRrQLcCCcfBBQzLBcvQBv
    wqnJjSmjrstdqwwFBLcRsBRRszzLFC
    qwdddTJTdHtjndqJqHZHmwVWGpDbGTlbWWpWWrPGhhhM
    WGllqLjjLCpSffmBmvfpHs
    dnrQwZzRTdZwnCThdzzFTVmcBHBJBmsHfBPHcfvcSVHs
    QgQrzCdrTRCZzrZLbjGLqNMWGgNNLt
    sgPnhPPTTPTTwlJfwNHlqcfs
    LMCpFbLLbRpMGbMcCFLVlNlNqrHqVfbHHwNDwr
    GjBcCCtWMtMRZTSvgWQTngvg
    BCMtJJMpRDlMMvBJBBnfjtcjPhPmZgnhgdcf
    NrsrsqFNvrVLVGVrsHsqFgfmcPGdcmhfjdPgfjcnZd
    zFTzsNqHqFssLVLQqNTFbsBDwCCwvWlDwRMRCTRBDMDS
    zQtLgvggSRtgvVRtLvvnzdnjnGwGdmmrlpnlGz
    JssBFpqsDqPNnlWWjrrjqrnj
    DHDFBNDfPbJBsFHNMPvpvStQvMRVTtgVTVtv
    FvzttFvBTJJzLbvwhCnnVnWwjCnBNC
    mQdZgZPDPdPPSsMSQPdZgCwVGmnwnWpGnGhqNWjWCG
    ggdDgfQSdcjtFHjlLJfF
    ghcgScNNSsCvGSzmpVFlZbrzcFcV
    MWWRLRqqqdQwTtLjjmqMlFpFlzVnbFVDwplFzlDr
    LHMHqdHWjdQMdMtLHHLtWjJRsGCGSNghmSvPBJBNhsGfvfGP
    CbVqqqDbcbMHnnDqcCbrRFCfBvvwGjzrBwQGzrwwBjGwBQ
    sTPmpNWdWPTJssSSLPfNljjBvflGtjwwBzMG
    mmWgmgSZLTLMZWpnhqZbhFFCnhqnnn
    QQmjmZqnmQrfTZlbbcVbBcfbHfzf
    vpdSNShNppFdSRtdGBqvJBDlDzqbPPHVBH
    tRNSNRFhNpSRhFRMFtGhRGswLZZsZqWnmrmZwqwsTZmmmQ
    gGWCllFCGWtGGWdlGlWNZdwpnnSbwpMvpphZpndn
    RsshDDLcQVMSJQwJwnvw
    HVPzrPcDNhPFGhPC
    jtHQGHjGGtdTLjnqTQlmvRPRPBBwRBnFPPWP
    hZbzNzVrczZzcbNssVspZZVvBwbmPmJPWmvbBRvPlmvRJF
    fzNVDsZMhzpVhpVhlZcMNfcDDdQTLTjGDTCqGCjtSQHdHL
    GrbFggGrTrzSrgfwJjdTmwmNJZJd
    VMPQplPDptchwdsjmlml
    MqMWtBDPPWDWHQtvqQtWPjbzCGLgSBgGbzgrzFgnnz
    fcJccCcwcDfcpbRnCfWJnQJqtqtqPQdsGdgPsgTQqg
    LSjVMhzSFFrljdNbltNGtgdqQq
    MMhSHFFMLzBWDcHHcfcHwb
    rwmWtJWMwSNRJMtwNmMrrSsmtTjjlgqnTqZZZPlHnTngTTgn
    BGqGqqFBFggjjdGHlj
    QDhhLbDQCDFMNcmhRhqJNW
    BnRnRvMnLGLSCHvvSnlRfWbbTNQJsJsbNbJTBfQT
    tzMmmMwjhcpFjDmMcptrcjzFQggfQPTsWsfgNbbgfhJbPhQT
    FdzcrtDwDMtcwtFGRZdRLvdnHRSZZv
    HVpsSpvjpNjsBmbGFBnMNnDM
    WRRWhZtfrVtLJrBZMnDmDbnZBTGF
    thhPLzWzhzwPtLRLWrQlpPvvClcVcCppSvpl
    lZPbhnZLRPnnPZZPdlGMBWcBMgMQHBBcvvvzBL
    jpFjmwwwCDDbsjvjjgcvQgcNBQ
    rbFmppbwhqhGRGZr
    ggrLwFgWCBwbMWBbFwLMgNBZdmZHclJPllnJlNRPmSNZRR
    ppszzDfhDfhsqpnvDVTfGpSPlPmclHcdRcZmmmdPPGSP
    pvtDDVDVpqDfzDfngBLCwQrgCtCwFwrg
    pbGjFFGGDjpbsGsmNhNFNRBBBtRhhhHv
    JnczJVCvwWJvhPgghgNtNtNJ
    nwVSSzdzzqSpvQSZQG
    mssLLttQrsMrMzLCRmMmrrSQpvWpDNlBTBDlvNTccDQl
    HdHJwJqVPwHnqJwbjJbGjnSgSTWPpNgWWpgBBgcvDWWN
    ZHVwVZGwwdndqJVJqfHbGwnwrRLtLMftMvMMRrhmLMthhLmz
    RgHGLbTqlZlPRZPHfvvfZttJnvfvjnzr
    sVcChDVDccwNhhvjTvVzWJjnzFff
    mpNcCMTCGmLqBLGH
    wVJwHJHVMtMpBmDDWPQVPWDGDD
    zCrlZzCblBvnCDWNGLmvGDLPNG
    dqZglgbzrzbbgZqzTFSBHHFJSSSfjjSMfwhj
    NMWJSjLMCnHHNMNNHWCHMbVVGBPZTrPVPBVDrBSDGTTr
    zvttlFpgdtldwwvftPDPTWQdBZrsrWrGBZ
    hFlFmhRFvfCbmWJWHcnj"
}